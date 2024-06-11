use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tracing::info;

mod config;
mod log;
mod metrics;

struct HandlerState {
    counter: prometheus::IntCounter,
}

#[tokio::main]
async fn main() {
    // load config
    let app_config = config::load();

    // init log
    log::init(app_config.log_level);

    info!("welcome to rust-start!");

    let shared_state = Arc::new(HandlerState {
        counter: metrics::create_counter("count", "the counter help"),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/count", get(count))
        .with_state(shared_state);

    tokio::spawn(async move {
        metrics::start_metrics_server(app_config.metrics_host, app_config.metrics_port).await;
    });

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", app_config.api_host, app_config.api_port))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn count(State(state): State<Arc<HandlerState>>) -> &'static str {
    state.counter.inc();

    "count"
}
