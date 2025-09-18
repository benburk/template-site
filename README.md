# Basic Site

A simple website to use as a starting point for saas projects.

The app is a single process, not dependent on any external resources.

## Tech stack

- Rust
- [Axum](https://github.com/tokio-rs/axum) for web server
- [sqlx](https://github.com/launchbadge/sqlx) for database connection
- [Askama](https://askama.readthedocs.io/en/stable/) for templating
- [HTMX](https://htmx.org) for reactivity
- [PicoCSS](https://picocss.com/docs/) for styling
- sqlite for database
- [justfile](https://just.systems/man/en/) for development recipes

## Features

- [x] Log in, log out, sign up
- [x] Password hashing with Argon2id
- [x] Change username / password
- [x] Request tracing in logs
- [ ] Session management
- [ ] Docker deployment

## Build and run instructions

1. run `nix develop`
2. create `.env`

```
MIGRATIONS_PATH=migrations
DATABASE_PATH=database.sqlite3
DATABASE_URL=sqlite:${DATABASE_PATH}
```

3. Run `just db-init`
4. Run `just run`
