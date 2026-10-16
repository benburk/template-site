use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use sqlx::{FromRow, SqliteExecutor};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: i64,
}

impl User {
    /// Inserts the user into the database and returns the new user's ID.
    pub async fn insert<'e, E: SqliteExecutor<'e>>(
        db: E,
        user: &Self,
    ) -> Result<(), sqlx::Error> {
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

    pub async fn get_by_username<'e, E: SqliteExecutor<'e>>(
        db: E,
        username: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT
            id as "id: uuid::Uuid",
            username,
            password_hash,
            email,
            created_at
            FROM 'user' WHERE username = ?"#,
            username
        )
        .fetch_one(db)
        .await
    }

    pub async fn get_by_id<'e, E: SqliteExecutor<'e>>(
        db: E,
        user_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT
            id as "id: uuid::Uuid",
            username,
            password_hash,
            email,
            created_at
            FROM 'user' WHERE id = ?"#,
            user_id
        )
        .fetch_one(db)
        .await
    }

    pub async fn update_email<'e, E: SqliteExecutor<'e>>(
        db: E,
        user_id: Uuid,
        email: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE user SET email = ? WHERE id = ?", email, user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    /// Checks a username and password against the database and returns the
    /// user if they match. Returns `None` if the user does not exist or the
    /// password is incorrect.
    pub async fn check_login<'e, E: SqliteExecutor<'e>>(
        db: E,
        username: &str,
        password: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let user = match Self::get_by_username(db, username).await {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(err) => return Err(err),
        };
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .expect("Failed to parse hash");
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .ok()
            .map(|()| user))
    }
}
