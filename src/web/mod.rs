//! Web routes.
use axum::{
    Router,
    http::header,
    response::IntoResponse,
    routing::{delete, get, post},
};

use crate::app_state::AppState;

mod about;
mod home;
mod html_template;
mod login;
mod profile;
mod session;
mod settings;
mod signup;

use about::about;
use home::home;
use profile::profile;

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
        .route("/users/{username}", get(profile))
        .route("/login", get(login::get))
        .route("/session", post(session::post).delete(session::delete))
        .route("/sessions/{session_id}", delete(session::delete_by_id))
        .route("/signup", get(signup::get).post(signup::post))
        .route("/settings", get(settings::get))
        .route("/settings/username", post(settings::update_username))
        .route("/settings/password", post(settings::update_password))
}
