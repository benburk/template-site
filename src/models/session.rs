use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: i64,
    pub expires_at: i64,
}

impl Session {
    pub async fn insert<'c, E: Executor<'c, Database = Sqlite>>(
        db: E,
        session: &Session,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO session (id, user_id, ip_address, user_agent, created_at, expires_at)
        VALUES (?, ?, ?, ?, ?, ?)",
            session.id,
            session.user_id,
            session.ip_address,
            session.user_agent,
            session.created_at,
            session.expires_at,
        )
        .execute(db)
        .await
        .map(|row| row.last_insert_rowid())
    }
}
