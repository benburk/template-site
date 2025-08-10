//! Web routes.
use axum::{Router, routing::get};

use crate::app_state::AppState;

mod home;
mod login;
mod signup;

use home::home;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route(
            "/login",
            get(login::get).post(login::post).delete(login::delete),
        )
        .route("/signup", get(signup::get).post(signup::post))
}
