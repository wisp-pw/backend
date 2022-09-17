use color_eyre::Result;

use axum_test_helper::TestClient;

pub async fn get_test_client() -> Result<TestClient> {
    use crate::{config::WispConfig, setup_app};
    use std::sync::Arc;

    let (router, _) = setup_app(Arc::new(WispConfig::for_test())).await?;
    let client = TestClient::new(router);
    Ok(client)
}
