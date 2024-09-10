use std::marker::PhantomData;

use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use rinja_axum::Template;

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
    }
}
