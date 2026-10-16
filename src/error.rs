//! The one error type handlers answer with.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use tracing::error;

use crate::web::pages;

/// Why a request got no ordinary answer. Handlers return it with `?`; a
/// refusal a person can fix, such as a bad form field, is no error here but
/// a message on the form they sent.
#[derive(Debug)]
pub enum AppError {
    /// Something broke on the server. The details go to the log, never to
    /// the visitor.
    Internal(anyhow::Error),
    /// No such page or record.
    NotFound,
    /// Signed in, but not allowed to do this.
    Forbidden,
    /// Not signed in, for something that needs an account: off to the
    /// login page.
    SignedOut,
    /// A request no page of the site sends, with a sentence saying what is
    /// wrong with it.
    BadRequest(String),
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError::Internal(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // The error pages are drawn with the signed-out header: an error
        // knows nothing of the request, and looking up the viewer here is not
        // worth it. Unknown addresses, the common case, are answered by
        // `web::not_found`, which does know the viewer.
        match self {
            AppError::Internal(err) => {
                error!("internal server error: {err:#}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong on the server.",
                )
                    .into_response()
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, pages::not_found("")).into_response()
            }
            AppError::Forbidden => {
                (StatusCode::FORBIDDEN, pages::forbidden("")).into_response()
            }
            AppError::SignedOut => {
                Redirect::to("/sessions/new").into_response()
            }
            AppError::BadRequest(why) => {
                (StatusCode::BAD_REQUEST, why).into_response()
            }
        }
    }
}
