use axum::response::IntoResponse;
use tracing::info;

use crate::models::user::User;

use super::pages;

pub async fn home(user_opt: Option<User>) -> impl IntoResponse {
    info!("Handling home request");
    let username = user_opt.map(|user| user.username).unwrap_or_default();
    pages::home(&username)
}
