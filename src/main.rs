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
use tokio::net::TcpListener;

mod models;
mod web;

use web::home::home;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Starting server on http://{addr}/");

    axum::serve(listener, app).await.unwrap();
}
