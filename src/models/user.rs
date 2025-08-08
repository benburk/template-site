use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Sqlite, SqliteConnection};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: i64,
}

impl User {
    pub async fn get_by_username(
        db: &mut SqliteConnection,
        username: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM 'user' WHERE username = ?")
            .bind(username)
            .fetch_one(db)
            .await
    }

    /// Checks a usernames+password combination using the database and returns the user if it is valid.
    /// Returns `None` if the user does not exist or the password is incorrect.
    pub async fn check_login(
        db: &mut SqliteConnection,
        username: &str,
        password: &str,
    ) -> Option<User> {
        match User::get_by_username(db, username).await {
            Ok(user) => {
                let parsed_hash =
                    PasswordHash::new(&user.password_hash).expect("Failed to parsh hash");
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .ok()
                    .map(|()| user)
            }
            Err(sqlx::Error::RowNotFound) => return None,
            Err(err) => {
                // TODO
                // error!(err = ?err, "Failed to get user");
                return None;
            }
        }
    }
}
