use askama::Template;
use axum::response::{Html, IntoResponse};
use tracing::info;

use crate::extractors::session::ExtractSession;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    username: String,
}

pub async fn home(user: Option<ExtractSession>) -> impl IntoResponse {
    info!("{user:?}");
    let template = Home {
        username: user.map(|u| u.0.username).unwrap_or_default(),
    };
    Html(template.render().unwrap())
}
