use axum::extract::{ConnectInfo, Path, State};
use axum::{Form, http::StatusCode, response::IntoResponse};
use axum_extra::TypedHeader;
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use axum_extra::headers::UserAgent;
use serde::Deserialize;
use sqlx::SqliteExecutor;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::internal_error;
use crate::models::{session::Session, user::User};
use crate::util::current_time_micros;

use super::login;

fn build_session_cookie(session_id: Uuid) -> Cookie<'static> {
    // This is a workaround so when we're testing locally
    // without https
    const IS_RELEASE: bool = !cfg!(debug_assertions);
    Cookie::build(("session_id", session_id.to_string()))
        .path("/")
        .same_site(SameSite::Strict)
        .secure(IS_RELEASE)
        .http_only(true)
        .max_age(time::Duration::WEEK)
        .build()
        .into_owned()
}

/// Create a new session for the user, insert it into the database,
/// and return the associated cookie for it.
pub async fn create_session<'e, E: SqliteExecutor<'e>>(
    db: E,
    user_id: Uuid,
    time: i64,
    ip_address: String,
    user_agent: UserAgent,
) -> Cookie<'static> {
    let id = uuid::Uuid::new_v4();
    let expire_micros =
        i64::try_from(time::Duration::WEEK.whole_microseconds()).unwrap();
    let session = Session {
        id,
        user_id,
        ip_address,
        user_agent: user_agent.to_string(),
        created_at: time,
        expires_at: time.wrapping_add(expire_micros),
    };
    Session::insert(db, &session).await.unwrap();
    build_session_cookie(id)
}

#[derive(Deserialize, Debug)]
pub struct CreateSessionPayload {
    username: String,
    password: String,
}

/// Create a new session (login)
pub async fn post(
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<AppState>,
    Form(form): Form<CreateSessionPayload>,
) -> impl IntoResponse {
    let created_at = current_time_micros();
    match User::check_login(&state.db, &form.username, &form.password).await {
        Some(user) => {
            let cookie = create_session(
                &state.db,
                user.id,
                created_at,
                addr.to_string(),
                user_agent,
            )
            .await;
            ([("HX-Redirect", "/")], jar.add(cookie)).into_response()
        }
        None => {
            login::login_form(&form.username, "Invalid username or password")
                .into_response()
        }
    }
}

/// Delete the current user's session (logout)
pub async fn delete(
    jar: CookieJar,
    state: State<AppState>,
    _user: Option<User>, // Triggers extractor to record session/user in span
) -> impl IntoResponse {
    if let Some(session_cookie) = jar.get("session_id")
        && let Ok(session_id) = Uuid::parse_str(session_cookie.value())
        && let Err(err) = Session::delete_by_id(&state.db, session_id).await
    {
        return internal_error(err).into_response();
    }
    (
        [("HX-Redirect", "/")],
        jar.remove(Cookie::build("session_id")),
    )
        .into_response()
}

/// Delete a specific session by session ID
pub async fn delete_by_id(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
    user_opt: Option<User>,
) -> impl IntoResponse {
    let Some(_user) = user_opt else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let Ok(session_uuid) = Uuid::parse_str(&session_id) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    // TODO: Verify the session belongs to the authenticated user
    // For now, we rely on the fact that session IDs are UUIDs and hard to guess

    match Session::delete_by_id(&state.db, session_uuid).await {
        Ok(_) => "".into_response(), // Return empty content to remove the row
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
