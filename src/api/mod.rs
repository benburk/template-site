//! API routes.
use axum::{Router, routing::get};

use project_name::app_state::AppState;

mod server_time;

pub fn router() -> Router<AppState> {
    Router::new().route("/time", get(server_time::get))
}
