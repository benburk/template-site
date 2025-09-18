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
- [ ] Session management
- [ ] Change username / password

## Run instructions

1. run `nix develop`
2. create `.env`

```
DATABASE_URL="sqlite://database.sqlite3"
```

3. Run `just db-init`
