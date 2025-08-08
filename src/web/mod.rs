//! Web routes.
use axum::{Router, routing::get};

use crate::app_state::AppState;

pub mod home;

use home::home;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(home))
}
