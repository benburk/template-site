use axum::{
    extract::{FromRequestParts as _, OptionalFromRequestParts},
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::Error;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    error::internal_error,
    models::{session::Session, user::User},
    util::current_time_micros,
};

impl OptionalFromRequestParts<AppState> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let Ok(jar) = CookieJar::from_request_parts(parts, state).await;

        let Some(raw) = jar.get("session_id").map(Cookie::value) else {
            info!("No session found");
            return Ok(None);
        };

        let Ok(session_id) = Uuid::parse_str(raw) else {
            warn!("Failed to parse session_id");
            return Ok(None);
        };

        info!("Extracting session {session_id}");

        let session = match Session::get_by_id(&state.db, session_id).await {
            Ok(Some(session)) => session,
            Ok(None) => return Ok(None),
            Err(err) => {
                return Err(internal_error(err));
            }
        };

        let current_time = current_time_micros();
        if session.expires_at <= current_time {
            warn!("Session {session_id} has expired");
            if let Err(err) = Session::delete_by_id(&state.db, session_id).await
            {
                warn!("Failed to delete expired session: {err}");
            }
            return Ok(None);
        }

        let user = match Self::get_by_id(&state.db, session.user_id).await {
            Ok(user) => user,
            Err(Error::RowNotFound) => return Ok(None),
            Err(err) => {
                return Err(internal_error(err));
            }
        };

        Ok(Some(user))
    }
}
