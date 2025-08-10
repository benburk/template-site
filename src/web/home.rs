use askama::Template;
use axum::response::IntoResponse;

use crate::models::user::User;

use super::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    username: String,
}

pub async fn home(user_opt: Option<User>) -> impl IntoResponse {
    let template = Home {
        username: user_opt.map(|user| user.username).unwrap_or_default(),
    };
    HtmlTemplate(template)
}
