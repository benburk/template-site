//! Form components for HTMX partial updates.
//!
//! These return HTML fragments, not full pages.
//! Use with hx-swap to replace form content on submit.

mod auth;
mod settings;

pub use auth::{login_form, signup_form};
pub use settings::{email_form, password_form, username_form};
