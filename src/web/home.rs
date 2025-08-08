use askama::Template;
use axum::response::{Html, IntoResponse};

use crate::middleware::auth::OptionalUser;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    user: Option<String>,
}

pub async fn home(OptionalUser(user): OptionalUser) -> impl IntoResponse {
    let template = Home {
        user: user.map(|u| u.username),
    };
    Html(template.render().unwrap())
}
