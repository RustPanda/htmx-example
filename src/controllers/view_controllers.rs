use std::marker::PhantomData;

use axum::{
    extract::{FromRef, State},
    response::{sse::Event, IntoResponse, Sse},
    routing::get,
    Router,
};
use futures::stream::{self, Stream};
use rinja_axum::Template;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;

use crate::use_cases::counter_use_case::CounterUseCase;

#[derive(Template)]
#[template(path = "counter.jinja", ext = "html")]
struct CounterTemplate {
    counter: i32,
}

#[derive(Template)]
#[template(path = "index.jinja", ext = "html")]
struct IndexTemplate {
    counter: i32,
}

pub async fn counter(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    let counter = use_case.get_value().await;

    CounterTemplate { counter }
}

pub async fn index(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    let counter = use_case.get_value().await;

    IndexTemplate { counter }
}

async fn sse_counter(
    State(use_case): State<CounterUseCase>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = BroadcastStream::new(use_case.subscribe()).map(|i| {
        let counter = i.unwrap();
        let template = CounterTemplate { counter }.render().unwrap();
        Ok(Event::default().event("CounterUpdate").data(template))
    });

    let first = stream::once(async move {
        let counter = use_case.get_value().await;
        let template = CounterTemplate { counter }.render().unwrap();
        Ok(Event::default().event("CounterUpdate").data(template))
    });

    Sse::new(first.chain(stream))
        .keep_alive(axum::response::sse::KeepAlive::new().text("keep-alive-text"))
}

pub struct ViewControllers<S>
where
    S: Clone + Send + Sync + 'static,
    CounterUseCase: FromRef<S>,
{
    d: PhantomData<S>,
}

impl<S> ViewControllers<S>
where
    S: Clone + Send + Sync + 'static,
    CounterUseCase: FromRef<S>,
{
    pub fn new() -> Router<S> {
        Router::new()
            .route("/", get(index))
            .route("/counter", get(counter))
            .route("/sse_counter", get(sse_counter))
    }
}
