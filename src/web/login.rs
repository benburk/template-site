use askama::Template;
use axum::response::{IntoResponse, Redirect};

use crate::models::user::User;

use super::html_template::HtmlTemplate;

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct Login {
    username: String,
    form: LoginForm,
}

#[derive(Template, Default)]
#[template(path = "login_form.html")]
pub struct LoginForm {
    pub username: String,
    pub error_message: String,
}

pub async fn get(user: Option<User>) -> impl IntoResponse {
    let Some(_) = user else {
        return HtmlTemplate(Login::default()).into_response();
    };
    Redirect::to("/").into_response()
}
