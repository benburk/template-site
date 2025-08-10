use axum_extra::{
    extract::cookie::{Cookie, SameSite},
    headers::UserAgent,
};
use sqlx::SqliteConnection;
use uuid::Uuid;

use crate::models::{session::Session, user::User};

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
pub async fn create_session(
    db: &mut SqliteConnection,
    user_id: Uuid,
    time: i64,
    ip_address: String,
    user_agent: UserAgent,
) -> Cookie<'static> {
    let id = uuid::Uuid::new_v4();
    let session = Session {
        id,
        user_id,
        ip_address,
        user_agent: user_agent.to_string(),
        created_at: time,
        expires_at: 0, // todo
    };
    Session::insert(db, &session).await.unwrap();
    build_session_cookie(id)
}

/// Authenticate user and create a new session id.
pub async fn login(
    db: &mut SqliteConnection,
    username: &str,
    password: &str,
    time: i64,
    ip_address: String,
    user_agent: UserAgent,
) -> Option<Cookie<'static>> {
    let user = User::check_login(db, username, password).await?;

    let cookie = create_session(db, user.id, time, ip_address, user_agent).await;
    Some(cookie)
}
