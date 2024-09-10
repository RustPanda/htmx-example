use std::marker::PhantomData;

use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use rinja_axum::Template;

use crate::{domain::models::Counter, use_cases::counter_use_case::CounterUseCase};

#[derive(Template)]
#[template(path = "counter.jinja", ext = "html")]
struct CounterTemplate {
    counter: i32,
}

#[derive(Template)]
#[template(path = "index.jinja", ext = "html")]
struct IndexTemplate<'a> {
    name: &'a str,
    counter: i32,
}

pub async fn counter(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    let Counter { value } = use_case.get().await;

    CounterTemplate { counter: value }
}

pub async fn index(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    let Counter { value } = use_case.get().await;

    IndexTemplate {
        name: "Maria",
        counter: value,
    }
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
