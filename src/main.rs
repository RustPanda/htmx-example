use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{extract::FromRef, Router};
use axum_embed::ServeEmbed;

use futures::stream::AbortHandle;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use use_cases::counter_use_case::CounterUseCase;

mod controllers;
mod domain;
mod repositories;
mod use_cases;

#[derive(Clone, Default)]
struct AbortableList(Arc<Mutex<std::collections::LinkedList<AbortHandle>>>);

impl AbortableList {
    fn push(&self, handler: AbortHandle) {
        self.0.lock().unwrap().push_back(handler)
    }

    fn abort(&self) {
        let mut list = self.0.lock().unwrap();
        while let Some(handler) = list.pop_back() {
            handler.abort();
        }
    }
}

#[derive(RustEmbed, Clone)]
#[folder = "static"]
struct Assets;

#[derive(FromRef, Clone)]
struct AppState {
    counter_use_case: CounterUseCase,
    abortable_list: AbortableList,
}

#[tokio::main]
async fn main() {
    tracing_subscriber_init().unwrap();

    let abortable_list = AbortableList::default();

    let counter_repository = repositories::in_memory_repository::InMemoryCounterRepository::new(0);
    let counter_use_case = use_cases::counter_use_case::CounterUseCase::new(counter_repository);

    let state = AppState {
        counter_use_case,
        abortable_list: abortable_list.clone(),
    };

    // Create a regular axum app.
    let app = Router::new()
        .nest("/api/counter", controllers::CounterControllers::new())
        .nest("/", controllers::ViewControllers::new())
        .fallback_service(ServeEmbed::<Assets>::new())
        .with_state(state)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Server run on: http://localhost:8080");

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(abortable_list))
        .await
        .unwrap();
}

async fn shutdown_signal(abortable_list: AbortableList) {
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
    abortable_list.abort();
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
