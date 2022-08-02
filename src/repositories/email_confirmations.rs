use sqlx::SqlitePool;
use uuid::Uuid;

use crate::prelude::*;

pub struct EmailConfirmationRepository {}

impl EmailConfirmationRepository {
    pub async fn create(pool: &SqlitePool, user_id: i32) -> Result<String> {
        let code = Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO email_confirmations (token, user) VALUES ($1, $2)")
            .bind(code.clone())
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(code)
    }
}
