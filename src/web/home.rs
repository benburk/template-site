use askama::Template;
use axum::response::{Html, IntoResponse};
use tracing::info;

use crate::models::user::User;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    username: String,
}

pub async fn home(user_opt: Option<User>) -> impl IntoResponse {
    info!("{user_opt:?}");
    let template = Home {
        username: user_opt.map(|user| user.username).unwrap_or_default(),
    };
    Html(template.render().unwrap())
}
