use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use serde::Deserialize;
use tracing::error;

use crate::app_state::AppState;
use crate::models::user::User;
use crate::password;

use super::html_template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct Settings {
    username: String,
    username_form: UsernameForm,
    password_form: PasswordForm,
}

#[derive(Template, Default)]
#[template(path = "settings_username_form.html")]
struct UsernameForm {
    new_username: String,
    username_message: String,
}

#[derive(Template, Default)]
#[template(path = "settings_password_form.html")]
struct PasswordForm {
    current_password_message: String,
    new_password_message: String,
}

pub async fn get(user_opt: Option<User>) -> impl IntoResponse {
    match user_opt {
        Some(user) => HtmlTemplate(Settings {
            username: user.username,
            username_form: UsernameForm::default(),
            password_form: PasswordForm::default(),
        })
        .into_response(),
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
        return HtmlTemplate(UsernameForm {
            new_username: form.new_username,
            username_message: username_error,
        })
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
            HtmlTemplate(UsernameForm {
                new_username: form.new_username.clone(),
                username_message: String::from(
                    "Username updated successfully!",
                ),
            }),
        )
            .into_response(),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            HtmlTemplate(UsernameForm {
                new_username: form.new_username,
                username_message: String::from("Username already taken"),
            })
            .into_response()
        }
        Err(err) => {
            error!("Failed to update username: {}", err);
            HtmlTemplate(UsernameForm {
                new_username: form.new_username,
                username_message: String::from("Failed to update username"),
            })
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
        return HtmlTemplate(PasswordForm {
            current_password_message: String::new(),
            new_password_message: password_error,
        })
        .into_response();
    }

    let valid_login =
        User::check_login(&state.db, &user.username, &form.current_password)
            .await;
    if valid_login.is_none() {
        return HtmlTemplate(PasswordForm {
            current_password_message: String::from(
                "Current password is incorrect",
            ),
            new_password_message: String::new(),
        })
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
        Ok(_) => HtmlTemplate(PasswordForm {
            current_password_message: String::new(),
            new_password_message: String::from(
                "Password updated successfully!",
            ),
        })
        .into_response(),
        Err(err) => {
            error!("Failed to update password: {}", err);
            HtmlTemplate(PasswordForm {
                current_password_message: String::new(),
                new_password_message: String::from("Failed to update password"),
            })
            .into_response()
        }
    }
}
