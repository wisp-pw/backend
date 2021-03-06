#[cfg(test)]
use color_eyre::Result;

#[cfg(test)]
use axum_test_helper::TestClient;

#[cfg(test)]
pub async fn get_test_client() -> Result<TestClient> {
    use crate::{settings::WispSettings, setup_app};
    use std::sync::Arc;

    let (router, _) = setup_app(Arc::new(WispSettings::for_test())).await?;
    let client = TestClient::new(router);
    Ok(client)
}
