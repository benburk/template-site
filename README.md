# Template Site

A template for full-stack Rust web applications. Start a new project with authentication, database, and server-side rendering already working—just rename and customize.

## Setup Instructions

Rename the package to match your project:

1. Update `name` in `Cargo.toml`
2. Update the site title in `src/web/components/layout.rs`
3. Update the tracing filter in `src/main.rs`

Then start building:

- Add models to `src/models/`
- Add migrations to `migrations/`
- Add pages to `src/web/pages.rs`
- Add routes in `src/web/mod.rs`

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

```bash
just              # Show available recipes
just run          # Start server
just watch        # Hot-reload development
just check        # Lint and format
just db-init      # Reset database
```

## Tech Stack

[Rust](https://www.rust-lang.org/) • [Axum](https://github.com/tokio-rs/axum) • [SQLite](https://sqlite.org/) • [sqlx](https://github.com/launchbadge/sqlx) • [MAUD](https://maud.lambda.xyz/) • [HTMX](https://htmx.org/) • [PicoCSS](https://picocss.com/)
