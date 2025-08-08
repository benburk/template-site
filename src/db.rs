use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect_to_database() -> SqlitePool {
    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqlitePoolOptions::new()
        .connect(&url)
        .await
        .expect("Failed to connect to database")
}
