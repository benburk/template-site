#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction
)]
#![allow(
    clippy::allow_attributes_without_reason,
    clippy::arbitrary_source_item_ordering,
    clippy::blanket_clippy_restriction_lints,
    clippy::cargo_common_metadata,
    clippy::expect_used,
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::multiple_crate_versions,
    clippy::print_stdout,
    clippy::question_mark_used,
    clippy::single_call_fn,
    clippy::single_char_lifetime_names,
    clippy::std_instead_of_core,
    clippy::unwrap_used
)]
use std::net::SocketAddr;

use axum::Router;
use db::connect_to_database;
use tokio::net::TcpListener;

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
use tracing::{info, subscriber};
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
