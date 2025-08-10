# Basic Site

A simple website to use as a starting point for saas projects.

## Tech stack

- Rust
- Axum for web server
- sqlx for database connection
- Askama for templating
- HTMX for reactivity
- PicoCSS for styling
- sqlite for database
- justfile for development recipes
- docker for deployment

## Features

- [x] Log in, log out, sign up
- [x] Password hashing with Argon2id
- Session management
- Change username / password

## Setup

```
cargo install sqlx-cli --no-default-features --features sqlite
cargo install --locked bacon
```
