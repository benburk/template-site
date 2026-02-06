use std::net::SocketAddr;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use basic_site::app_state::AppState;
use basic_site::db::connect_to_database;
use basic_site::services;
use basic_site::web;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing::field;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;

fn configure_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new("basic_site=info,tower_http=info")
                })
                .unwrap(),
        )
        .init();
}

#[tokio::main]
async fn main() {
    configure_logging();

    let db = connect_to_database().await;

    let (job_tx, job_rx) = mpsc::unbounded_channel();
    tokio::spawn(services::job::run(db.clone(), job_rx));

    let state = AppState { db, job_tx };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let request_id = uuid::Uuid::new_v4();
            tracing::info_span!(
                "request",
                %request_id,
                method = %request.method(),
                uri = %request.uri(),
                session_id = field::Empty,
                user_id = field::Empty,
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::DEBUG))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .merge(web::static_router())
        .merge(web::router().layer(trace_layer.clone()))
        .nest("/api/v1", api::router().layer(trace_layer))
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
