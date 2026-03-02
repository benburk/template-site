set dotenv-load := true

# Display available recipes
default:
    just --list --unsorted

# Install dependencies and set up the development environment
bootstrap:
    cargo build

alias fmt := format

# Format code
format:
    dprint fmt
    cargo fmt
    fd -e nix | xargs -r nixfmt
    rg -l '[^\n]\z' --multiline | xargs -r sed -i -e '$a\\'

# Run linters and static analysis
check:
    dprint check
    cargo fmt --check
    cargo clippy -- -D warnings
    fd -e nix | xargs -r nixfmt --check
    ! rg -l '[^\n]\z' --multiline

# Run the test suite
test:
    cargo test

# Build release binary
build:
    cargo build --release

# Run the project
run *args:
    cargo run -- {{ args }}

# Watch for changes and restart
watch:
    watchexec -r -e rs,html,css,js -- cargo run

# Reset database with migrations and seeds
db-init:
    sqlx database drop -y
    sqlx database create
    sqlx migrate run
    sqlite3 $DATABASE_PATH < seeds/seed.sql
