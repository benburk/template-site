set dotenv-load

check:
    #!/usr/bin/env fish
    set status_flag 0
    dprint check --config ~/.config/dprint/dprint.json; or set status_flag 1
    cargo fmt --check; or set status_flag 1
    cargo clippy; or set status_flag 1
    fd -e nix | xargs nixfmt --check; or set status_flag 1
    exit $status_flag
    
format:
    dprint fmt --config ~/.config/dprint/dprint.json
    cargo fmt
    fd -e nix | xargs nixfmt

db-init:
    sqlx database drop
    sqlx database create
    sqlx migrate run

