use maud::{DOCTYPE, Markup, html};

pub fn base(username: &str, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Basic Site" }
                link rel="stylesheet" href="/pico.min.css";
                link rel="stylesheet" href="/pico.colors.min.css";
                script src="/htmx.min.js" {}
            }
            body {
                (navbar(username))
                main class="container" {
                    (content)
                }
            }
        }
    }
}

pub fn navbar(username: &str) -> Markup {
    html! {
        nav class="container-fluid" {
            ul {
                li {
                    h1 { a href="/" { "Basic Site" } }
                }
            }
            ul {
                li { a href="/about" { "About" } }
                @if username.is_empty() {
                    li { a href="/login" { "Log in" } }
                    li { a href="/signup" { "Sign up" } }
                } @else {
                    li { a href={ "/users/" (username) } { (username) } }
                    li { a href="/settings" { "Settings" } }
                    li {
                        a href="/" hx-delete="/session" hx-trigger="click" hx-swap="none" class="secondary" {
                            "Logout"
                        }
                    }
                }
            }
        }
    }
}
