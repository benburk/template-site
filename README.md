# Template Site

A forkable template for full-stack Rust web applications. Start a new project with authentication, database, and server-side rendering already working—just rename and customize.

## Quick Start

```bash
# Fork on GitHub, then clone your fork
git clone git@github.com:YOUR_USERNAME/YOUR_PROJECT.git
cd YOUR_PROJECT

# Set up environment
nix develop
just db-init
just run
```

Visit http://localhost:3000 to see the running app with login, signup, and settings pages.

## Features

- **Server-side rendering** with MAUD (type-safe HTML) and HTMX (interactivity without JS frameworks)
- **Authentication** with Argon2 password hashing and cookie-based sessions
- **SQLite database** with sqlx compile-time query validation
- **Background jobs** via Tokio channels (no external queue needed)
- **Single binary** deployment—no external services required
- **PicoCSS** for styling semantic HTML without utility classes

## After Forking

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
