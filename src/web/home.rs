use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::extractors::session::ExtractSession;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    user: Option<String>,
}

pub async fn home(user: Option<ExtractSession>) -> impl IntoResponse {
    let template = Home {
        user: user.map(|u| u.0.username),
    };
    Html(template.render().unwrap())
}
