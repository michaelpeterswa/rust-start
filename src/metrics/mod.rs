use axum::{routing::get, Router};
use prometheus::{Encoder, TextEncoder};

pub async fn start_metrics_server(host: String, port: u16) {
    let app = Router::new()
        .route("/metrics", get(prometheus))
        .route("/healthcheck", get(healthcheck));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn prometheus() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

async fn healthcheck() -> String {
    gethostname::gethostname().into_string().unwrap()
}

pub fn create_counter(t: &str, h: &str) -> prometheus::IntCounter {
    let counter_opts = prometheus::Opts::new(t, h);
    let counter = prometheus::IntCounter::with_opts(counter_opts).unwrap();
    prometheus::register(Box::new(counter.clone())).unwrap();

    counter
}
