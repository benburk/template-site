use axum::Form;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;
use tracing::error;

use crate::app_state::AppState;
use crate::models::user::User;
use crate::password;

use super::templates;

pub async fn get(user_opt: Option<User>) -> impl IntoResponse {
    match user_opt {
        Some(user) => templates::settings(&user.username).into_response(),
        None => Redirect::to("/login").into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateUsernamePayload {
    new_username: String,
}

#[derive(Deserialize)]
pub struct UpdatePasswordPayload {
    current_password: String,
    new_password: String,
}

pub async fn update_username(
    State(state): State<AppState>,
    user_opt: Option<User>,
    Form(form): Form<UpdateUsernamePayload>,
) -> impl IntoResponse {
    let Some(user) = user_opt else {
        return Redirect::to("/login").into_response();
    };

    let username_error = password::validate_username(&form.new_username);
    if !username_error.is_empty() {
        return templates::username_form(
            &form.new_username,
            &username_error,
            false,
        )
        .into_response();
    }

    let query_result = sqlx::query!(
        "UPDATE user SET username = ? WHERE id = ?",
        form.new_username,
        user.id
    )
    .execute(&state.db)
    .await;

    match query_result {
        Ok(_) => (
            [("HX-Trigger", "username-updated")],
            templates::username_form(
                &form.new_username,
                "Username updated successfully!",
                true,
            ),
        )
            .into_response(),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            templates::username_form(
                &form.new_username,
                "Username already taken",
                false,
            )
            .into_response()
        }
        Err(err) => {
            error!("Failed to update username: {}", err);
            templates::username_form(
                &form.new_username,
                "Failed to update username",
                false,
            )
            .into_response()
        }
    }
}

pub async fn update_password(
    State(state): State<AppState>,
    user_opt: Option<User>,
    Form(form): Form<UpdatePasswordPayload>,
) -> impl IntoResponse {
    let Some(user) = user_opt else {
        return Redirect::to("/login").into_response();
    };

    let password_error = password::validate_password(&form.new_password);
    if !password_error.is_empty() {
        return templates::password_form("", &password_error, false, false)
            .into_response();
    }

    let valid_login =
        User::check_login(&state.db, &user.username, &form.current_password)
            .await;
    if valid_login.is_none() {
        return templates::password_form(
            "Current password is incorrect",
            "",
            false,
            false,
        )
        .into_response();
    }

    let new_password_hash = password::generate_hash(&form.new_password);

    let query_result = sqlx::query!(
        "UPDATE user SET password_hash = ? WHERE id = ?",
        new_password_hash,
        user.id
    )
    .execute(&state.db)
    .await;

    match query_result {
        Ok(_) => templates::password_form(
            "",
            "Password updated successfully!",
            false,
            true,
        )
        .into_response(),
        Err(err) => {
            error!("Failed to update password: {}", err);
            templates::password_form(
                "",
                "Failed to update password",
                false,
                false,
            )
            .into_response()
        }
    }
}
