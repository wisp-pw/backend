use crate::prelude::*;

pub async fn get() -> impl IntoResponse {
    "hello tiny gay person"
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, setup_app};

    #[tokio::test]
    async fn index() -> Result<()> {
        let (router, _) = setup_app().await.unwrap();

        let client = TestClient::new(router);
        let response = client.get("/").send().await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await, "hello tiny gay person");

        Ok(())
    }
}
