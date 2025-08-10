//! Web routes.
use axum::{Router, routing::{get, post}};

use crate::app_state::AppState;

mod home;
mod login;
mod logout;
mod signup;

use home::home;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/login", get(login::get).post(login::post))
        .route("/logout", post(logout::post))
        .route("/signup", get(signup::get).post(signup::post))
}
