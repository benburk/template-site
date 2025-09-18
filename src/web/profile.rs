use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::models::user::User;

use super::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "profile.html")]
struct Profile {
    username: String,
}

pub async fn profile(
    Path(username): Path<String>,
    _user_opt: Option<User>,
) -> impl IntoResponse {
    let template = Profile { username };
    HtmlTemplate(template)
}
