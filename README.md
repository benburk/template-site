# Template Site

A starting point for full-stack Rust web applications that constrains LLM-assisted development to a known-good stack.

## Features

- **Server-side rendering** with [MAUD](https://maud.lambda.xyz/) (type-safe HTML via Rust macros) and [fixi.js](https://github.com/bigskysoftware/fixi) (interactivity without JS frameworks)
- **Authentication** with [Argon2](https://en.wikipedia.org/wiki/Argon2) password hashing and cookie-based sessions
- **SQLite database** with [sqlx](https://github.com/launchbadge/sqlx) compile-time query validation
- **Background jobs** via [Tokio](https://tokio.rs/) channels (no external queue needed)
- **Single binary** deployment — no external services required
- **[PicoCSS](https://picocss.com/)** for styling semantic HTML without utility classes

## Setup Instructions

Two places carry the project's name; the rest refer to the library as `app`:

1. Update `name` in `Cargo.toml`
2. Update `APP_NAME` in `src/lib.rs` — the name shown to visitors

Then:

```sh
cp .env.example .env
nix develop --command just check      # no database needed
nix develop --command just db-init    # only to run the app
```

## Queries and the database

`sqlx` validates SQL at compile time. Rather than requiring a live database
to build, the query metadata is committed to `.sqlx/` and `SQLX_OFFLINE=true`
makes the macros read it — so building, linting, and CI need no database at
all.

After adding or changing a query, refresh it:

```sh
nix develop --command just prepare    # needs a database; run db-init first
```

Forgetting is safe: the next build fails with `no cached data for this
query` rather than silently disagreeing with what is committed.

## Project Structure

```
src/
├── main.rs              # Entry point, spawns background services
├── app_state.rs         # Shared state (db pool, job channel)
├── models/              # Database models (Active Record pattern)
├── services/            # Background job processors
├── web/
│   ├── components/      # MAUD components (HTML fragments for fixi)
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
- [fixi.js](https://github.com/bigskysoftware/fixi)
- [PicoCSS](https://picocss.com/)
- [direnv](https://direnv.net/)
- [just](https://github.com/casey/just)
