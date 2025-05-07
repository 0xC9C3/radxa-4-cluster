use crate::metrics::get_fan_metrics;
use axum::body::Body;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use log::info;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use tokio::sync::Mutex;

// via https://github.com/prometheus/client_rust/blob/master/examples/axum.rs

#[derive(Debug)]
pub struct AppState {
    pub registry: Registry,
}

pub async fn metrics_handler(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().await;
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(Body::from(buffer))
        .unwrap()
}

pub async fn health() -> impl IntoResponse {
    "ok".to_string()
}

pub async fn run_metrics_server() {
    let mut state = AppState {
        registry: Registry::default(),
    };
    state.registry.register(
        "temperature",
        "Device temperature",
        get_fan_metrics().temperature.clone(),
    );
    state.registry.register(
        "fan_speed",
        "Fan speed",
        get_fan_metrics().fan_speed.clone(),
    );

    let state = Arc::new(Mutex::new(state));

    let router = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state)
        .route("/health", get(health));
    //.with_state(metrics);
    let port = std::env::var("METRICS_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let listening_address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&listening_address)
        .await
        .unwrap();

    info!("Metrics server running on {}", listening_address);
    axum::serve(listener, router).await.unwrap();
}
