use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::prelude::*;
use crate::services::jwt::JwtService;
use crate::config::WispConfig;

pub struct WispState {
    pub sql_pool: SqlitePool,
    pub jwt_service: JwtService,
}

impl WispState {
    pub async fn new(settings: &Arc<WispConfig>) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(&settings.db_uri).await?;
        sqlx::migrate!().run(&pool).await?;

        let jwt_service = JwtService::new(&settings.jwt.secret);

        Ok(WispState {
            sql_pool: pool,
            jwt_service,
        })
    }
}
