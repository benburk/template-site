set dotenv-load

check:
    #!/usr/bin/env fish
    set status_flag 0
    cargo fmt --check; or set status_flag 1
    cargo clippy; or set status_flag 1
    exit $status_flag
    
format:
    cargo fmt

db-init:
    sqlx database drop
    sqlx database create
    sqlx migrate run

