use askama::Template;
use axum::response::{Html, IntoResponse};
use tracing::info;

use crate::models::user::User;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    username: String,
}

pub async fn home(user: Option<User>) -> impl IntoResponse {
    info!("{user:?}");
    let template = Home {
        username: user.map(|u| u.username).unwrap_or_default(),
    };
    Html(template.render().unwrap())
}
