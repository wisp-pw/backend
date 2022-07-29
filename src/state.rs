use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::prelude::*;
use crate::settings::WispSettings;

pub struct WispState {
    pub sql_pool: SqlitePool,
}

impl WispState {
    pub async fn new(settings: &Arc<WispSettings>) -> Result<WispState> {
        let pool = SqlitePoolOptions::new().connect(&settings.db_uri).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(WispState { sql_pool: pool })
    }
}
