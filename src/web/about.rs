use askama::Template;
use axum::response::IntoResponse;

use crate::models::user::User;

use super::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "about.html")]
struct About {
    username: String,
}

pub async fn about(user_opt: Option<User>) -> impl IntoResponse {
    let template = About {
        username: user_opt.map(|user| user.username).unwrap_or_default(),
    };
    HtmlTemplate(template)
}
