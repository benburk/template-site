use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    error::internal_error,
    models::{session::Session, user::User},
};

impl OptionalFromRequestParts<AppState> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let mut conn = state.db.acquire().await.map_err(internal_error)?;
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();

        let Some(session_id) = jar.get("session_id").map(Cookie::value) else {
            info!("No session found");
            return Ok(None);
        };

        let Ok(session_id) = Uuid::parse_str(session_id) else {
            warn!("Failed to parse session_id");
            return Ok(None);
        };

        info!("Extracting session {session_id}");

        let session = match Session::get_by_id(&mut conn, session_id).await {
            Ok(Some(session)) => session,
            Ok(None) => return Ok(None),
            Err(err) => {
                warn!("{err}");
                return Err(internal_error(err));
            }
        };

        let user = match User::get_by_id(&mut conn, session.user_id).await {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => {
                warn!("{err}");
                return Err(internal_error(err));
            }
        };

        Ok(Some(user))
    }
}
