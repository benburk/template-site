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
    clippy::std_instead_of_core,
    clippy::unwrap_used
)]
use std::net::SocketAddr;

use axum::{Router, routing::get};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::net::TcpListener;

mod models;
mod web;

use web::home::home;

pub async fn connect_to_database() -> SqlitePool {
    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePoolOptions::new()
        .connect(&url)
        .await
        .expect("Failed to connect to database")
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let db = connect_to_database().await;

    let state = AppState { db };

    let app = Router::new().route("/", get(home)).with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Starting server on http://{addr}/");

    axum::serve(listener, app).await.unwrap();
}
