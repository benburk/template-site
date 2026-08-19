//! A forkable Rust web application template.

/// The name shown to visitors. Distinct from the package name, which is
/// kebab-case and never displayed.
pub const APP_NAME: &str = "Project Name";

pub mod app_state;
pub mod db;
pub mod error;
pub mod extractors;
pub mod models;
pub mod password;
pub mod services;
pub mod util;
pub mod web;
