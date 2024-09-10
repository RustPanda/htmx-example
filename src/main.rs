use std::time::Duration;

use axum::{extract::FromRef, Router};
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use use_cases::counter_use_case::CounterUseCase;

mod controllers;
mod domain;
mod repositories;
mod use_cases;

#[derive(FromRef, Clone)]
struct AppState {
    counter_use_case: CounterUseCase,
}

#[tokio::main]
async fn main() {
    tracing_subscriber_init().unwrap();

    let counter_repository = repositories::in_memory_repository::InMemoryCounterRepository::new(0);
    let counter_use_case = use_cases::counter_use_case::CounterUseCase::new(counter_repository);

    let state = AppState { counter_use_case };

    // Create a regular axum app.
    let app = Router::new()
        .nest("/", controllers::ViewControllers::new())
        .nest("/api/counter", controllers::CounterControllers::new())
        .with_state(state)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn tracing_subscriber_init() -> Result<(), tracing_subscriber::util::TryInitError> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .try_init()
}
