//! Full page templates.
//!
//! These return complete HTML documents wrapped in the base layout.
//! For fixi partial updates, use components directly.

use maud::{Markup, html};

use crate::APP_NAME;

use super::components::{
    SessionDisplay, base, email_form, login_form, password_form, signup_form,
    username_form,
};

pub fn home(username: &str) -> Markup {
    base(
        username,
        APP_NAME,
        "/",
        &html! {
            h1 { (APP_NAME) }
            p { "A simple web application built with modern Rust tooling." }
            h2 { "Tech Stack" }
            ul {
                li { "Rust" }
                li { a href="https://github.com/tokio-rs/axum" { "Axum" } " for web server" }
                li { a href="https://github.com/launchbadge/sqlx" { "sqlx" } " for database connection" }
                li { "Maud for html components" }
                li { a href="https://github.com/bigskysoftware/fixi" { "fixi.js" } " for reactivity" }
                li { a href="https://picocss.com/docs/" { "PicoCSS" } " for styling" }
                li { "sqlite for database" }
            }
        },
    )
}

pub fn about(username: &str) -> Markup {
    base(
        username,
        "About",
        "/about",
        &html! {
            h1 { "About" }
            p { "Hello World" }
        },
    )
}

pub fn login_page() -> Markup {
    base("", "Log in", "/sessions/new", &login_form("", ""))
}

pub fn signup_page() -> Markup {
    base("", "Sign up", "/users/new", &signup_form("", "", ""))
}

pub fn settings(username: &str, email: Option<&str>) -> Markup {
    base(
        username,
        "Settings",
        "/settings",
        &html! {
            h1 { "Settings" }
            section {
                (username_form("", "", false))
            }
            section {
                (email_form(email.unwrap_or(""), "", false))
            }
            section {
                details {
                    summary { "Change password" }
                    (password_form("", "", false, false))
                }
            }
        },
    )
}

pub fn profile(username: &str, sessions: &[SessionDisplay]) -> Markup {
    let profile_path = format!("/users/{username}");
    base(
        username,
        "Profile",
        &profile_path,
        &html! {
            h1 { "Hello, " (username) "!" }
            h2 id="active-sessions-heading" tabindex="-1" { "Active Sessions" }
            @if !sessions.is_empty() {
                table {
                    thead {
                        tr {
                            th scope="col" { "Device/Browser" }
                            th scope="col" { "IP Address" }
                            th scope="col" { "Created" }
                            th scope="col" { "Expires" }
                            th scope="col" { "Actions" }
                        }
                    }
                    tbody {
                        @for session in sessions {
                            tr id={ "session-" (session.id) }
                                data-theme=[session.is_current.then_some("primary")] {
                                td { (session.user_agent) }
                                td { (session.ip_address) }
                                td { (session.created_at) }
                                td { (session.expires_at) }
                                td {
                                    @if session.is_current {
                                        small { "Current session" }
                                    } @else {
                                        button type="button"
                                            fx-action={"/sessions/" (session.id)}
                                            fx-method="delete"
                                            fx-target={"#session-" (session.id)}
                                            ext-fx-confirm="Are you sure you want to revoke this session?"
                                            data-focus-after-remove="#active-sessions-heading"
                                            data-theme="outline"
                                        {
                                            "Revoke"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } @else {
                p { "No active sessions found." }
            }
        },
    )
}
