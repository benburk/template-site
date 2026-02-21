# Template Site

A starting point for full-stack Rust web applications that constrains LLM-assisted development to a known-good stack.

## Features

- **Server-side rendering** with [MAUD](https://maud.lambda.xyz/) (type-safe HTML via Rust macros) and [HTMX](https://htmx.org/) (interactivity without JS frameworks)
- **Authentication** with [Argon2](https://en.wikipedia.org/wiki/Argon2) password hashing and cookie-based sessions
- **SQLite database** with [sqlx](https://github.com/launchbadge/sqlx) compile-time query validation
- **Background jobs** via [Tokio](https://tokio.rs/) channels (no external queue needed)
- **Single binary** deployment — no external services required
- **[PicoCSS](https://picocss.com/)** for styling semantic HTML without utility classes

## Setup Instructions

Rename the package to match your project:

1. Update `name` in `Cargo.toml`
2. Update the site title in `src/web/components/layout.rs`
3. Update the tracing filter in `src/main.rs`
4. Run `direnv allow && just bootstrap && just check`

## Project Structure

```
src/
├── main.rs              # Entry point, spawns background services
├── app_state.rs         # Shared state (db pool, job channel)
├── models/              # Database models (Active Record pattern)
├── services/            # Background job processors
├── web/
│   ├── components/      # MAUD components (HTML fragments for HTMX)
│   ├── pages.rs         # Full page templates
│   └── [feature].rs     # Route handlers
└── extractors/          # Custom Axum extractors (auth)
static/                  # CSS/JS embedded at compile time
migrations/              # SQLx migrations
```

## Development

Run `just` to see available recipes.

## Tech Stack

- [Axum](https://github.com/tokio-rs/axum)
- [SQLite](https://sqlite.org/)
- [sqlx](https://github.com/launchbadge/sqlx)
- [MAUD](https://maud.lambda.xyz/)
- [HTMX](https://htmx.org/)
- [PicoCSS](https://picocss.com/)
- [direnv](https://direnv.net/)
- [just](https://github.com/casey/just)
