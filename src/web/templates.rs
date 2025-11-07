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

fn navbar(username: &str) -> Markup {
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

pub fn login_form(username: &str, error_message: &str) -> Markup {
    html! {
        article {
            form hx-post="/session" hx-swap="outerHTML" method="post" {
                h1 { "Login" }
                fieldset {
                    @if error_message.is_empty() {
                        label {
                            "Username "
                            input name="username" type="text" placeholder="Username" required;
                        }
                        label {
                            "Password "
                            input name="password" type="password" placeholder="Password" required;
                        }
                    } @else {
                        label {
                            "Username "
                            input name="username" type="text" placeholder="Username" required aria-invalid="true" value=(username);
                        }
                        label {
                            "Password "
                            input name="password" type="password" placeholder="Password" aria-invalid="true" required;
                            small { (error_message) }
                        }
                    }
                }
                button type="submit" { "Log in" }
                p { "Don't have an account? " a href="/signup" { "Sign up" } }
            }
        }
    }
}

pub fn signup_page() -> Markup {
    base("", &signup_form("", "", ""))
}

pub fn signup_form(
    username: &str,
    username_message: &str,
    password_message: &str,
) -> Markup {
    html! {
        article {
            form hx-post="/signup" hx-swap="outerHTML" method="post" {
                h1 { "Sign up" }
                fieldset {
                    label {
                        "Username"
                        @if username_message.is_empty() {
                            input name="username" type="text" placeholder="Username" value=(username) required;
                        } @else {
                            input name="username" type="text" placeholder="Username" value=(username) required aria-invalid="true";
                            small { (username_message) }
                        }
                    }
                    label {
                        "Password"
                        @if password_message.is_empty() {
                            input name="password" type="password" placeholder="Password" required;
                        } @else {
                            input name="password" type="password" placeholder="Password" required aria-invalid="true";
                            small { (password_message) }
                        }
                    }
                }
                button type="submit" { "Sign up" }
                p { "Already have an account? " a href="/login" { "Log in" } }
            }
        }
    }
}

pub fn settings(username: &str) -> Markup {
    base(
        username,
        &html! {
            h1 { "Settings" }
            section {
                (username_form("", "", false))
            }
            section {
                (password_form("", "", false, false))
            }
        },
    )
}

pub fn username_form(
    new_username: &str,
    username_message: &str,
    username_is_success: bool,
) -> Markup {
    html! {
        form hx-post="/settings/username" hx-swap="outerHTML" method="post" action="/settings/username" {
            label for="new_username" {
                "New Username"
                @if username_message.is_empty() {
                    input type="text" id="new_username" name="new_username" placeholder="Enter new username" value=(new_username) required minlength="5" maxlength="20";
                } @else {
                    input type="text" id="new_username" name="new_username" placeholder="Enter new username" value=(new_username) required minlength="5" maxlength="20" aria-invalid=(if username_is_success { "false" } else { "true" });
                    small { (username_message) }
                }
            }
            button type="submit" { "Update Username" }
        }
    }
}

pub fn password_form(
    current_password_message: &str,
    new_password_message: &str,
    current_password_is_success: bool,
    new_password_is_success: bool,
) -> Markup {
    html! {
        form hx-post="/settings/password" hx-swap="outerHTML" method="post" action="/settings/password" {
            label for="current_password" {
                "Current Password"
                @if current_password_message.is_empty() {
                    input type="password" id="current_password" name="current_password" placeholder="Enter current password" required;
                } @else {
                    input type="password" id="current_password" name="current_password" placeholder="Enter current password" required aria-invalid=(if current_password_is_success { "false" } else { "true" });
                    small { (current_password_message) }
                }
            }
            label for="new_password" {
                "New Password"
                @if new_password_message.is_empty() {
                    input type="password" id="new_password" name="new_password" placeholder="Enter new password" required minlength="8" maxlength="60";
                } @else {
                    input type="password" id="new_password" name="new_password" placeholder="Enter new password" required minlength="8" maxlength="60" aria-invalid=(if new_password_is_success { "false" } else { "true" });
                    small { (new_password_message) }
                }
            }
            button type="submit" { "Update Password" }
        }
    }
}

pub struct SessionDisplay {
    pub id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: String,
    pub expires_at: String,
    pub is_current: bool,
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
