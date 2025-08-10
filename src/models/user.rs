use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use sqlx::{FromRow, SqliteConnection};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: i64,
}

impl User {
    /// Inserts the user into the database and returns the new user's ID.
    pub async fn insert(db: &mut SqliteConnection, user: &Self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO user (id, username, password_hash, created_at) VALUES (?, ?, ?, ?)",
            user.id,
            user.username,
            user.password_hash,
            user.created_at
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn get_by_username(
        db: &mut SqliteConnection,
        username: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT
            id as "id: uuid::Uuid",
            username,
            password_hash,
            created_at
            FROM 'user' WHERE username = ?"#,
            username
        )
        .fetch_one(db)
        .await
    }

    pub async fn get_by_id(db: &mut SqliteConnection, user_id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT
            id as "id: uuid::Uuid",
            username,
            password_hash,
            created_at
            FROM 'user' WHERE id = ?"#,
            user_id
        )
        .fetch_one(db)
        .await
    }

    /// Checks a usernames+password combination using the database and returns the user if it is valid.
    /// Returns `None` if the user does not exist or the password is incorrect.
    pub async fn check_login(
        db: &mut SqliteConnection,
        username: &str,
        password: &str,
    ) -> Option<Self> {
        match Self::get_by_username(db, username).await {
            Ok(user) => {
                let parsed_hash =
                    PasswordHash::new(&user.password_hash).expect("Failed to parsh hash");
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .ok()
                    .map(|()| user)
            }
            Err(sqlx::Error::RowNotFound) => None,
            Err(_err) => None,
        }
    }
}
