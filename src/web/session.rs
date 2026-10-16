use axum::extract::{ConnectInfo, Path, State};
use axum::{
    Form,
    response::{IntoResponse, Redirect, Response},
};
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
use crate::error::AppError;
use crate::extractors::session::SignedIn;
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
) -> Result<Cookie<'static>, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    // This constant conversion always succeeds (1 week in microseconds fits in i64)
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
    Session::insert(db, &session).await?;
    Ok(build_session_cookie(id))
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
) -> Result<Response, AppError> {
    let created_at = current_time_micros();
    let Some(user) =
        User::check_login(&state.db, &form.username, &form.password).await?
    else {
        return Ok(login::login_form(
            &form.username,
            "Invalid username or password",
        )
        .into_response());
    };

    let cookie = create_session(
        &state.db,
        user.id,
        created_at,
        addr.to_string(),
        user_agent,
    )
    .await?;
    Ok((jar.add(cookie), Redirect::to("/")).into_response())
}

/// Delete the current user's session (logout)
pub async fn delete(
    jar: CookieJar,
    state: State<AppState>,
    _user: Option<User>, // Triggers extractor to record session/user in span
) -> Result<impl IntoResponse, AppError> {
    if let Some(session_cookie) = jar.get("session_id")
        && let Ok(session_id) = Uuid::parse_str(session_cookie.value())
    {
        Session::delete_by_id(&state.db, session_id).await?;
    }
    Ok((jar.remove(Cookie::build("session_id")), Redirect::to("/")))
}

/// Delete a specific session by session ID
pub async fn delete_by_id(
    Path(session_id): Path<String>,
    State(state): State<AppState>,
    SignedIn(user): SignedIn,
) -> Result<(), AppError> {
    let Ok(session_uuid) = Uuid::parse_str(&session_id) else {
        return Err(AppError::BadRequest(format!(
            "`{session_id}` is not a session id."
        )));
    };

    // Verify the session belongs to the authenticated user
    let session = Session::get_by_id(&state.db, session_uuid)
        .await?
        .ok_or(AppError::NotFound)?;
    if session.user_id != user.id {
        return Err(AppError::Forbidden);
    }

    Session::delete_by_id(&state.db, session_uuid).await?;
    Ok(())
}
