//! Authentication form components (login, signup).

use maud::{Markup, html};

pub fn login_form(username: &str, error_message: &str) -> Markup {
    let has_error = !error_message.is_empty();
    html! {
        article hx-target="this" hx-swap="outerHTML" {
            header { h1 { "Login" } }
            form hx-post="/session" method="post" {
                fieldset {
                    label {
                        "Username "
                        input name="username" type="text" placeholder="Username" required autofocus autocomplete="username"
                            value=[has_error.then_some(username)]
                            aria-invalid=[has_error.then_some("true")];
                    }
                    label {
                        "Password "
                        input name="password" type="password" placeholder="Password" required autocomplete="current-password"
                            aria-invalid=[has_error.then_some("true")];
                        @if has_error {
                            small { (error_message) }
                        }
                    }
                }
                button type="submit" { "Log in" }
            }
            footer { "Don't have an account? " a href="/signup" { "Sign up" } }
        }
    }
}

pub fn signup_form(
    username: &str,
    username_message: &str,
    password_message: &str,
) -> Markup {
    html! {
        article hx-target="this" hx-swap="outerHTML" {
            header { h1 { "Sign up" } }
            form hx-post="/signup" method="post" {
                fieldset {
                    label {
                        "Username"
                        input name="username" type="text" placeholder="Username" value=(username) required autofocus autocomplete="username"
                            aria-invalid=[(!username_message.is_empty()).then_some("true")];
                        @if !username_message.is_empty() {
                            small { (username_message) }
                        }
                    }
                    label {
                        "Password"
                        input name="password" type="password" placeholder="Password" required autocomplete="new-password"
                            aria-invalid=[(!password_message.is_empty()).then_some("true")];
                        @if !password_message.is_empty() {
                            small { (password_message) }
                        }
                    }
                }
                button type="submit" { "Sign up" }
            }
            footer { "Already have an account? " a href="/login" { "Log in" } }
        }
    }
}
