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
};

impl OptionalFromRequestParts<AppState> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();

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
