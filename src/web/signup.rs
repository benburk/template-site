use axum::extract::{ConnectInfo, State};
use axum::{
    Form,
    response::{IntoResponse, Redirect},
};
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::headers::UserAgent;
use serde::Deserialize;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::internal_error;
use crate::models::user::User;
use crate::password;
use crate::util::current_time_micros;

use super::{components, pages};

pub async fn get(user: Option<User>) -> impl IntoResponse {
    let Some(_) = user else {
        return pages::signup_page().into_response();
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
    State(state): State<AppState>,
    Form(form): Form<FormPayload>,
) -> impl IntoResponse {
    if let Err((username_message, password_message)) = validate_inputs(&form) {
        return components::signup_form(
            &form.username,
            &username_message,
            &password_message,
        )
        .into_response();
    }

    let created_at = current_time_micros();
    let password_hash = password::generate_hash(&form.password);

    let uuid = Uuid::new_v4();
    let user = User {
        id: uuid,
        username: form.username.clone(),
        password_hash,
        email: None,
        created_at,
    };
    match User::insert(&state.db, &user).await {
        Ok(user_id) => user_id,
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            return components::signup_form(
                &form.username,
                "Username already taken",
                "",
            )
            .into_response();
        }
        Err(err) => return internal_error(err).into_response(),
    }

    match super::session::create_session(
        &state.db,
        user.id,
        created_at,
        addr.to_string(),
        user_agent,
    )
    .await
    {
        Ok(cookie) => ([("HX-Redirect", "/")], jar.add(cookie)).into_response(),
        Err(err) => internal_error(err).into_response(),
    }
}

fn validate_inputs(form: &FormPayload) -> Result<(), (String, String)> {
    let username_message = password::validate_username(&form.username);
    let password_message = password::validate_password(&form.password);
    if !username_message.is_empty() || !password_message.is_empty() {
        Err((username_message, password_message))
    } else {
        Ok(())
    }
}
