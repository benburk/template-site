use sqlx::SqliteConnection;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct Invite {
    pub id: String,
    pub used_by: Option<String>,
    pub created_by: String,
    pub created_at: i64,
}

impl Invite {
    pub async fn check_and_claim(
        db: &mut SqliteConnection,
        id: &str,
        user_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Invite>(
            "UPDATE invite 
             SET used_by = ? 
             WHERE id = ? AND used_by IS NULL
             RETURNING *",
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(db)
        .await
    }
}
