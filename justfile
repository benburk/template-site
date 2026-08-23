set dotenv-load

# Display available recipes
default:
    just --list --unsorted

# Install dependencies and set up the development environment
bootstrap:
    cargo build

alias fmt := format

# Format code
format:
    just --fmt
    dprint fmt
    cargo fmt --all
    fd -e nix -X nixfmt
    # The trailing `.` is required: with no path, ripgrep reads stdin when
    # stdin is not a TTY and blocks forever instead of searching the tree.
    rg -l '[^\n]\z' --multiline . | xargs -r sed -i -e '$a\\'

# Run linters and static analysis
check:
    just --fmt --check
    dprint check
    @fd -e md --hidden -E .git -X awk '/^[[:space:]]*[|]/ && length($0) > 80 {print FILENAME ":" FNR ": table row is " length($0) " chars (>80)"; bad=1} END {exit bad}'
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    fd -e nix -X nixfmt --check
    ! rg -l '[^\n]\z' --multiline .

# Run the test suite
test:
    cargo test --workspace

# Build release binary
build:
    cargo build --release

# Run the server until interrupted
serve:
    cargo run

# Restart the server when source files change
watch:
    watchexec --restart --watch src --watch static --exts rs,html,css,js -- just serve

# Needs a live database; everything else does not.
[doc('Refresh the offline query metadata in .sqlx/ after changing a query')]
prepare:
    cargo sqlx prepare

# Reset database with migrations and seeds
db-init:
    sqlx database drop -y
    sqlx database create
    sqlx migrate run
    sqlite3 $DATABASE_PATH < seeds/seed.sql
