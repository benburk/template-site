use std::net::SocketAddr;

use axum::Router;
use db::connect_to_database;
use tokio::net::TcpListener;
use tower_http::LatencyUnit;
use tower_http::trace::{self, TraceLayer};

mod api;
mod app_state;
mod db;
mod error;
mod extractors;
mod models;
mod session;
mod util;
mod web;

use app_state::AppState;
use tracing::{Level, info, subscriber};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

fn configure_logging() {
    let file_appender =
        RollingFileAppender::new(Rotation::DAILY, "logs", "basic-site.log");

    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_ansi(false)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Multiple global default subscribers set");
}

#[tokio::main]
async fn main() {
    configure_logging();

    let db = connect_to_database().await;

    let state = AppState { db };

    let app = Router::new()
        .merge(web::router())
        .nest("/api/v1", api::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    trace::DefaultMakeSpan::new().level(Level::INFO),
                )
                .on_response(
                    trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
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
