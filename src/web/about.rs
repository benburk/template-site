use axum::response::IntoResponse;

use crate::models::user::User;

use super::templates;

pub async fn about(user_opt: Option<User>) -> impl IntoResponse {
    let username = user_opt.map(|user| user.username).unwrap_or_default();
    templates::about(&username)
}
