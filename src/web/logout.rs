use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;

pub async fn post(
    jar: CookieJar,
) -> impl IntoResponse {
    let jar = jar.remove(Cookie::from("session_id"));
    (jar, Redirect::to("/")).into_response()
}