//! Web routes.
use axum::{Router, http::header, response::IntoResponse, routing::get};

use crate::app_state::AppState;

mod about;
mod home;
mod html_template;
mod login;
mod signup;

use about::about;
use home::home;

async fn get_pico_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("../../static/pico.min.css"),
    )
}

async fn get_htmx() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/javascript")],
        include_str!("../../static/htmx.min.js"),
    )
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pico.min.css", get(get_pico_css))
        .route("/htmx.min.js", get(get_htmx))
        .route("/", get(home))
        .route("/about", get(about))
        .route(
            "/login",
            get(login::get).post(login::post).delete(login::delete),
        )
        .route("/signup", get(signup::get).post(signup::post))
}
