use askama::Template;
use axum::extract::{ConnectInfo, State};
use axum::{
    Form,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::headers::UserAgent;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::warn;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::models::user::User;
use crate::password;
use crate::util::current_time_micros;

use super::html_template::HtmlTemplate;

#[derive(Template, Default)]
#[template(path = "signup.html")]
pub struct Signup {
    username: String,
    form: SignupForm,
}

#[derive(Template, Default)]
#[template(path = "signup_form.html")]
pub struct SignupForm {
    username: String,
    username_message: String,
    password_message: String,
}

/// Get the signup page, or redirect to the home page if the user is already logged in.
pub async fn get(user: Option<User>) -> impl IntoResponse {
    let Some(_) = user else {
        return HtmlTemplate(Signup::default()).into_response();
    };
    Redirect::to("/").into_response()
}

#[derive(Deserialize, Debug)]
pub struct FormPayload {
    username: String,
    password: String,
}

/// Handle a signup request.
pub async fn post(
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Form(form): Form<FormPayload>,
) -> impl IntoResponse {
    if let Err(page) = validate_inputs(&form) {
        return HtmlTemplate(page).into_response();
    }

    let created_at = current_time_micros();
    let password_hash = password::generate_hash(&form.password);

    let uuid = Uuid::new_v4();
    let user = User {
        id: uuid,
        username: form.username.clone(),
        password_hash,
        created_at,
    };
    match User::insert(&state.db, &user).await {
        Ok(user_id) => user_id,
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            return HtmlTemplate(SignupForm {
                username: form.username,
                username_message: String::from("Username already taken"),
                password_message: String::new(),
            })
            .into_response();
        }
        Err(err) => {
            warn!("{err}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let cookie = super::session::create_session(
        &state.db,
        user.id,
        created_at,
        addr.to_string(),
        user_agent,
    )
    .await;

    ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
}

fn validate_inputs(form: &FormPayload) -> Result<(), SignupForm> {
    let username_message = password::validate_username(&form.username);
    let password_message = password::validate_password(&form.password);
    if !username_message.is_empty() || !password_message.is_empty() {
        Err(SignupForm {
            username: form.username.clone(),
            username_message,
            password_message,
        })
    } else {
        Ok(())
    }
}
