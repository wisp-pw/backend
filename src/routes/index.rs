use crate::prelude::*;

pub async fn get() -> impl IntoResponse {
    "hello tiny gay person"
}

#[cfg(test)]
mod tests {
    use crate::{prelude::*, router};

    use axum::{body::Body, http::Request};

    #[tokio::test]
    async fn index_test() -> Result<()> {
        let (router, _) = router().await.unwrap();

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await?;
        assert_eq!(&body[..], b"hello tiny gay person");

        Ok(())
    }
}
