set dotenv-load

check:
    cargo fmt --check
    cargo clippy
    
format:
    cargo fmt

db-init:
    sqlx database drop
    sqlx database create
    sqlx migrate run

