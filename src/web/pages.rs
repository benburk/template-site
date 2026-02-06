//! Full page templates.
//!
//! These return complete HTML documents wrapped in the base layout.
//! For HTMX partial updates, use components directly.

use maud::{Markup, html};

use super::components::{
    SessionDisplay, base, email_form, login_form, password_form, signup_form,
    username_form,
};

pub fn home(username: &str) -> Markup {
    base(
        username,
        &html! {
            h1 { "Basic Site" }
            p { "A simple web application built with modern Rust tooling." }
            h2 { "Tech Stack" }
            ul {
                li { "Rust" }
                li { a href="https://github.com/tokio-rs/axum" { "Axum" } " for web server" }
                li { a href="https://github.com/launchbadge/sqlx" { "sqlx" } " for database connection" }
                li { "Maud for html components" }
                li { a href="https://htmx.org" { "HTMX" } " for reactivity" }
                li { a href="https://picocss.com/docs/" { "PicoCSS" } " for styling" }
                li { "sqlite for database" }
            }
        },
    )
}

pub fn about(username: &str) -> Markup {
    base(
        username,
        &html! {
            h1 { "Hello World" }
        },
    )
}

pub fn login_page() -> Markup {
    base("", &login_form("", ""))
}

pub fn signup_page() -> Markup {
    base("", &signup_form("", "", ""))
}

pub fn settings(username: &str, email: Option<&str>) -> Markup {
    base(
        username,
        &html! {
            h1 { "Settings" }
            section {
                (username_form("", "", false))
            }
            section {
                (email_form(email.unwrap_or(""), "", false))
            }
            section {
                (password_form("", "", false, false))
            }
        },
    )
}

pub fn profile(username: &str, sessions: &[SessionDisplay]) -> Markup {
    base(
        username,
        &html! {
            h1 { "Hello, " (username) "!" }
            h2 { "Active Sessions" }
            @if !sessions.is_empty() {
                table {
                    thead {
                        tr {
                            th { "Device/Browser" }
                            th { "IP Address" }
                            th { "Created" }
                            th { "Expires" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        @for session in sessions {
                            tr data-theme=[session.is_current.then_some("primary")] {
                                td { (session.user_agent) }
                                td { (session.ip_address) }
                                td { (session.created_at) }
                                td { (session.expires_at) }
                                td {
                                    @if session.is_current {
                                        small { "Current session" }
                                    } @else {
                                        button
                                            hx-delete={"/sessions/" (session.id)}
                                            hx-target="closest tr"
                                            hx-swap="outerHTML swap:1s"
                                            hx-confirm="Are you sure you want to revoke this session?"
                                            data-theme="outline"
                                            role="button"
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
