use axum::response::IntoResponse;

use crate::models::user::User;

use super::pages;

pub async fn home(user_opt: Option<User>) -> impl IntoResponse {
    let username = user_opt.map(|user| user.username).unwrap_or_default();
    pages::home(&username)
}
