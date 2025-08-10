use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher as _, password_hash::SaltString};
use askama::Template;
use axum::extract::{ConnectInfo, State};
use axum::response::Html;
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
use tracing::{info, warn};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::extractors::db_connection::DatabaseConnection;
use crate::extractors::session::ExtractSession;
use crate::models::user::User;
use crate::session::create_session;
use crate::util::current_time_micros;

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
pub async fn get(user: Option<ExtractSession>) -> impl IntoResponse {
    let Some(_) = user else {
        return Html(Signup::default().render().unwrap()).into_response();
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
    DatabaseConnection(mut conn): DatabaseConnection,
    State(_state): State<AppState>,
    Form(form): Form<FormPayload>,
) -> impl IntoResponse {
    info!("signup::post");
    if let Err(page) = validate_inputs(&form) {
        return page.render().unwrap().into_response();
    }

    let created_at = current_time_micros();

    let password_hash = generate_password_hash(&form.password);

    let uuid = Uuid::new_v4();
    let user = User {
        id: uuid,
        username: form.username.clone(),
        password_hash,
        created_at,
    };
    match User::new(&mut conn, &user).await {
        Ok(user_id) => user_id,
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            return Html(
                SignupForm {
                    username: form.username,
                    username_message: String::from("Username already taken"),
                    password_message: String::new(),
                }
                .render()
                .unwrap(),
            )
            .into_response();
        }
        Err(err) => {
            warn!("{err}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let cookie = create_session(&mut conn, user.id, created_at, addr.to_string(), user_agent).await;

    ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
}

fn generate_password_hash(plaintext_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plaintext_password.as_bytes(), &salt)
        .unwrap();
    password_hash.to_string()
}

fn validate_inputs(form: &FormPayload) -> Result<(), SignupForm> {
    let username_message = validate_username(&form.username);
    let password_message = validate_password(&form.password);
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

fn validate_username(username: &str) -> String {
    if username.len() < 5 || username.len() > 20 || !username.chars().all(char::is_alphanumeric) {
        String::from(
            "Username must be between 5 and 20 characters, and only contain letters / numbers.",
        )
    } else {
        String::new()
    }
}

fn validate_password(password: &str) -> String {
    if password.len() < 8 || password.len() > 60 || !password.is_ascii() {
        String::from(
            "Password must be between 8 and 60 characters and only contain ascii characters.",
        )
    } else {
        String::new()
    }
}
