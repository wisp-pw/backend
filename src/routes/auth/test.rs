#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        routes::auth::{login::LoginRequest, register::RegisterRequest},
    };

    #[derive(Serialize, Deserialize)]
    struct ErrorResponse {
        pub error: String,
    }

    #[tokio::test]
    async fn login_unregistered_email() -> Result<()> {
        let client = get_test_client().await?;

        let request = LoginRequest {
            email_or_username: "me@lily.lol".to_string(),
            password: "boobies".to_string(),
        };

        let response = client.post("/auth/login").json(&request).send().await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response = response.json::<ErrorResponse>().await;
        assert_eq!(&response.error, "INVALID_EMAIL");

        Ok(())
    }

    #[tokio::test]
    async fn login_unregistered_username() -> Result<()> {
        let client = get_test_client().await?;

        let request = LoginRequest {
            email_or_username: "lily".to_string(),
            password: "boobies".to_string(),
        };

        let response = client.post("/auth/login").json(&request).send().await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response = response.json::<ErrorResponse>().await;
        assert_eq!(&response.error, "INVALID_USERNAME");

        Ok(())
    }

    #[tokio::test]
    async fn login_invalid_password() -> Result<()> {
        let client = get_test_client().await?;

        // register valid account
        {
            let request = RegisterRequest {
                email: "meow@meow.com".to_string(),
                username: "lily".to_string(),
                password: "boobies".to_string(),
            };

            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // then attempt an invalid login
        {
            let request = LoginRequest {
                email_or_username: "lily".to_string(),
                password: "boob".to_string(),
            };

            let response = client.post("/auth/login").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);

            let response = response.json::<ErrorResponse>().await;
            assert_eq!(&response.error, "INVALID_PASSWORD");
        }

        Ok(())
    }

    #[tokio::test]
    async fn login_valid() -> Result<()> {
        let client = get_test_client().await?;

        // register valid account
        {
            let request = RegisterRequest {
                email: "meow@meow.com".to_string(),
                username: "lily".to_string(),
                password: "boobies".to_string(),
            };

            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // then attempt an invalid login
        {
            let request = LoginRequest {
                email_or_username: "lily".to_string(),
                password: "boobies".to_string(),
            };

            let response = client.post("/auth/login").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::OK);
        }

        Ok(())
    }

    #[tokio::test]
    async fn register_invalid_email() -> Result<()> {
        let client = get_test_client().await?;

        let request = RegisterRequest {
            email: "meow.meow.com".to_string(),
            username: "lily".to_string(),
            password: "boobies".to_string(),
        };

        let response = client.post("/auth/register").json(&request).send().await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response = response.json::<ErrorResponse>().await;
        assert_eq!(&response.error, "INVALID_EMAIL");

        Ok(())
    }

    #[tokio::test]
    async fn register_already_registered_email() -> Result<()> {
        let client = get_test_client().await?;

        // valid registration
        {
            let request = RegisterRequest {
                email: "meow@meow.com".to_string(),
                username: "lily".to_string(),
                password: "boobies".to_string(),
            };
    
            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // should error due to email already being registered
        {
            let request = RegisterRequest {
                email: "meow@meow.com".to_string(),
                username: "lily2".to_string(),
                password: "boobies".to_string(),
            };
    
            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);

            let response = response.json::<ErrorResponse>().await;
            assert_eq!(&response.error, "EMAIL_USED");
        }

        Ok(())
    }

    #[tokio::test]
    async fn register_already_registered_username() -> Result<()> {
        let client = get_test_client().await?;

        // valid registration
        {
            let request = RegisterRequest {
                email: "meow@meow.com".to_string(),
                username: "lily".to_string(),
                password: "boobies".to_string(),
            };
    
            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // should error due to email already being registered
        {
            let request = RegisterRequest {
                email: "meow1@meow.com".to_string(),
                username: "lily".to_string(),
                password: "boobies".to_string(),
            };
    
            let response = client.post("/auth/register").json(&request).send().await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);

            let response = response.json::<ErrorResponse>().await;
            assert_eq!(&response.error, "USERNAME_USED");
        }

        Ok(())
    }

    #[tokio::test]
    async fn register_valid() -> Result<()> {
        let client = get_test_client().await?;

        let request = RegisterRequest {
            email: "meow@meow.com".to_string(),
            username: "lily".to_string(),
            password: "boobies".to_string(),
        };

        let response = client.post("/auth/register").json(&request).send().await;
        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }
}
