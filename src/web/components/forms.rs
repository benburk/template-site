//! Form components for HTMX partial updates.
//!
//! These return HTML fragments, not full pages.
//! Use with hx-swap to replace form content on submit.

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

pub fn username_form(
    new_username: &str,
    username_message: &str,
    username_is_success: bool,
) -> Markup {
    let aria_invalid = (!username_message.is_empty())
        .then_some(if username_is_success { "false" } else { "true" });
    html! {
        form hx-post="/settings/username" hx-swap="outerHTML" method="post" action="/settings/username" {
            label for="new_username" {
                "New Username"
                input type="text" id="new_username" name="new_username" placeholder="Enter new username" value=(new_username) required minlength="5" maxlength="20"
                    aria-invalid=[aria_invalid];
                @if !username_message.is_empty() {
                    small { (username_message) }
                }
            }
            button type="submit" { "Update Username" }
        }
    }
}

pub fn email_form(
    current_email: &str,
    message: &str,
    is_success: bool,
) -> Markup {
    let aria_invalid = (!message.is_empty()).then_some(if is_success {
        "false"
    } else {
        "true"
    });
    html! {
        form hx-post="/settings/email" hx-swap="outerHTML" method="post" action="/settings/email" {
            label for="email" {
                "Email"
                input type="email" id="email" name="email" placeholder="Enter email address" value=(current_email) autocomplete="email"
                    aria-invalid=[aria_invalid];
                @if !message.is_empty() {
                    small { (message) }
                }
            }
            button type="submit" { "Update Email" }
        }
    }
}

pub fn password_form(
    current_password_message: &str,
    new_password_message: &str,
    current_password_is_success: bool,
    new_password_is_success: bool,
) -> Markup {
    let current_aria = (!current_password_message.is_empty()).then_some(
        if current_password_is_success {
            "false"
        } else {
            "true"
        },
    );
    let new_aria = (!new_password_message.is_empty()).then_some(
        if new_password_is_success {
            "false"
        } else {
            "true"
        },
    );
    html! {
        form hx-post="/settings/password" hx-swap="outerHTML" method="post" action="/settings/password" {
            label for="current_password" {
                "Current Password"
                input type="password" id="current_password" name="current_password" placeholder="Enter current password" required
                    aria-invalid=[current_aria];
                @if !current_password_message.is_empty() {
                    small { (current_password_message) }
                }
            }
            label for="new_password" {
                "New Password"
                input type="password" id="new_password" name="new_password" placeholder="Enter new password" required minlength="8" maxlength="60"
                    aria-invalid=[new_aria];
                @if !new_password_message.is_empty() {
                    small { (new_password_message) }
                }
            }
            button type="submit" { "Update Password" }
        }
    }
}
