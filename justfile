DATABASE_PATH := "db.sqlite3"
DATABASE_URL := "sqlite:" + DATABASE_PATH


check:
    cargo fmt --check
    cargo clippy
    
format:
    cargo fmt

db-init:
    sqlx database drop --database-url {{DATABASE_URL}}
    sqlx database create --database-url {{DATABASE_URL}}
    sqlx migrate run --database-url {{DATABASE_URL}}

