use crate::{prelude::*, settings::WispSettings};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub async fn new(settings: &Arc<WispSettings>) -> Result<UserRepository> {
        let pool = SqlitePoolOptions::new().connect(&settings.db_uri).await?;
        Ok(UserRepository { pool })
    }
}
