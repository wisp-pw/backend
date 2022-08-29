use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::prelude::*;
use crate::services::jwt::JwtService;
use crate::settings::WispSettings;

pub struct WispState {
    pub sql_pool: SqlitePool,
    pub jwt_service: JwtService,
}

impl WispState {
    pub async fn new(settings: &Arc<WispSettings>) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(&settings.db_uri).await?;
        sqlx::migrate!().run(&pool).await?;

        let jwt_service = JwtService::new(&settings.jwt_secret);

        Ok(WispState { sql_pool: pool, jwt_service })
    }
}
