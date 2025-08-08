use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar, SameSite},
    headers::UserAgent,
};
use sqlx::{SqliteConnection, SqlitePool};
use std::future::Future;

use crate::{
    app_state::AppState,
    models::{Session, User},
    util::generate_hex_token,
};

fn build_session_cookie(session_id: &str) -> Cookie<'static> {
    // This is a workaround so when we're testing locally
    // without https
    const IS_RELEASE: bool = !cfg!(debug_assertions);
    Cookie::build(("session_id", session_id))
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
    user_id: String,
    time: i64,
    ip_address: String,
    user_agent: UserAgent,
) -> Cookie<'static> {
    let id = generate_hex_token();
    let session = Session {
        id: id.clone(),
        user_id,
        ip_address,
        user_agent: user_agent.to_string(),
        created_at: time,
        expires_at: 0, // todo
    };
    Session::insert(db, &session).await.unwrap();
    build_session_cookie(&id)
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

pub struct OptionalUser(pub Option<User>);

impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = ();

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let jar = CookieJar::from_request_parts(parts, state)
                .await
                .map_err(|_| ())?;

            let Some(session_cookie) = jar.get("session_id") else {
                return Ok(OptionalUser(None));
            };

            let session_id = session_cookie.value();

            let user = get_user_from_session(&state.db, session_id).await;

            Ok(OptionalUser(user))
        }
    }
}

async fn get_user_from_session(db: &SqlitePool, session_id: &str) -> Option<User> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;

    sqlx::query_as::<_, User>(
        "SELECT u.* FROM user u 
         JOIN session s ON u.id = s.user_id 
         WHERE s.id = ? AND (s.expires_at = 0 OR s.expires_at > ?)",
    )
    .bind(session_id)
    .bind(now)
    .fetch_one(db)
    .await
    .ok()
}
