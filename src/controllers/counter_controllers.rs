use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::use_cases::counter_use_case::CounterUseCase;

pub async fn increment(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    use_case.increment().await;
    StatusCode::OK
}

pub async fn decrement(State(use_case): State<CounterUseCase>) -> impl IntoResponse {
    use_case.decrement().await;
    StatusCode::OK
}

pub struct CounterControllers;

impl<S> Into<Router<S>> for CounterControllers
where
    S: 'static + Sync + Send + Clone,
    CounterUseCase: FromRef<S> {
    fn into(self) -> Router<S> {
        Router::new()
            .route("/increment", get(increment))
            .route("/decrement", get(decrement))
    }
}
