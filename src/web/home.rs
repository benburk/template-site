use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "home.html")]
struct Home;

pub async fn home() -> impl IntoResponse {
    Html(Home.render().unwrap())
}
