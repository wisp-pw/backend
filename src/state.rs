use crate::prelude::*;
use crate::{repositories::user::UserRepository, settings::WispSettings};

pub struct WispState {
    pub user_repository: UserRepository,
}

impl WispState {
    pub async fn new(settings: &Arc<WispSettings>) -> Result<WispState> {
        let user_repository = UserRepository::new(settings).await?;
        Ok(WispState { user_repository })
    }
}
