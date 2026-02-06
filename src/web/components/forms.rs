//! Form components for HTMX partial updates.
//!
//! These return HTML fragments, not full pages.
//! Use with hx-swap to replace form content on submit.

use maud::{Markup, html};

pub fn login_form(username: &str, error_message: &str) -> Markup {
    html! {
        article hx-target="this" hx-swap="outerHTML" {
            header { h1 { "Login" } }
            form hx-post="/session" method="post" {
                fieldset {
                    @if error_message.is_empty() {
                        label {
                            "Username "
                            input name="username" type="text" placeholder="Username" required autofocus autocomplete="username";
                        }
                        label {
                            "Password "
                            input name="password" type="password" placeholder="Password" required autocomplete="current-password";
                        }
                    } @else {
                        label {
                            "Username "
                            input name="username" type="text" placeholder="Username" required autofocus autocomplete="username" aria-invalid="true" value=(username);
                        }
                        label {
                            "Password "
                            input name="password" type="password" placeholder="Password" aria-invalid="true" required autocomplete="current-password";
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
                        @if username_message.is_empty() {
                            input name="username" type="text" placeholder="Username" value=(username) required autofocus autocomplete="username";
                        } @else {
                            input name="username" type="text" placeholder="Username" value=(username) required autofocus autocomplete="username" aria-invalid="true";
                            small { (username_message) }
                        }
                    }
                    label {
                        "Password"
                        @if password_message.is_empty() {
                            input name="password" type="password" placeholder="Password" required autocomplete="new-password";
                        } @else {
                            input name="password" type="password" placeholder="Password" required autocomplete="new-password" aria-invalid="true";
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
