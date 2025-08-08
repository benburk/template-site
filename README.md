# Basic Site

A simple website to use as a starting point for saas projects.

## Tech stack

- Rust
- Axum for web server
- sqlx for database access
- Askama for templating
- HTMX for reactivity
- PicoCSS for styling
- justfile for development recipes
- docker for deployment

## Features

- Log in, log out, sign up
- Password hashing with Argon2id
- Session management
- Change username / password

## Setup

```
cargo install sqlx-cli --no-default-features --features sqlite
cargo install --locked bacon
```
