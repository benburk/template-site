use std::net::SocketAddr;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use db::connect_to_database;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::field;

mod api;
mod app_state;
mod db;
mod error;
mod extractors;
mod models;
mod password;
mod services;
mod util;
mod web;

use app_state::AppState;
use tracing::{Level, info};
use tracing_subscriber::EnvFilter;

fn configure_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new(
                        "basic_site=debug,tower_http::trace=debug",
                    )
                })
                .unwrap(),
        )
        .init();
}

#[tokio::main]
async fn main() {
    configure_logging();

    let db = connect_to_database().await;

    let (job_tx, job_rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(services::job::run(db.clone(), job_rx));

    let state = AppState { db, job_tx };

    let app = Router::new()
        .merge(web::router())
        .nest("/api/v1", api::router())
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    method = field::display(request.method()),
                    uri = field::display(request.uri()),
                    version = field::debug(request.version()),
                    request_id = field::display(request_id)
                )
            },
        ))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    info!("Starting server on {addr}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Failed to serve");
}
