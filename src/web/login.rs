use askama::Template;
use axum::extract::{ConnectInfo, State};
use axum::response::Html;
use axum::{
    Form,
    response::{IntoResponse, Redirect},
};
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::headers::UserAgent;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::debug;

use crate::app_state::AppState;
use crate::extractors::db_connection::DatabaseConnection;
use crate::extractors::session::ExtractSession;
use crate::middleware::auth;
use crate::util::current_time_micros;

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct Login {
    username: String,
    form: LoginForm,
}

#[derive(Template, Default)]
#[template(path = "login_form.html")]
pub struct LoginForm {
    username: String,
    error_message: String,
}

pub async fn get(user: Option<ExtractSession>) -> impl IntoResponse {
    let Some(_) = user else {
        return Html(Login::default().render().unwrap()).into_response();
    };
    Redirect::to("/").into_response()
}

#[derive(Deserialize, Debug)]
pub struct FormPayload {
    username: String,
    password: String,
}

pub async fn post(
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    DatabaseConnection(mut conn): DatabaseConnection,
    State(_state): State<AppState>,
    Form(form): Form<FormPayload>,
) -> impl IntoResponse {
    let created_at = current_time_micros();
    debug!("post request");

    match auth::login(
        &mut conn,
        &form.username,
        &form.password,
        created_at,
        addr.to_string(),
        user_agent,
    )
    .await
    {
        Some(cookie) => ([("HX-Redirect", "/")], jar.add(cookie)).into_response(),
        None => Html(
            LoginForm {
                username: form.username,
                error_message: "Invalid username or password".to_string(),
            }
            .render()
            .unwrap(),
        )
        .into_response(),
    }
}
