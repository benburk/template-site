use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

use crate::{app_state::AppState, error::internal_error};

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Sqlite>);

impl FromRequestParts<AppState> for DatabaseConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let conn = state.db.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}
