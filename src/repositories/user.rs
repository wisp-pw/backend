use crate::prelude::*;
use sqlx::SqlitePool;

pub struct UserRepository {}

impl UserRepository {
    pub async fn create_user(
        pool: &SqlitePool,
        display_name: &str,
        email: &str,
        password_hash: &str,
        auto_confirm: bool,
    ) -> Result<i32> {
        let id: (i32,) = sqlx::query_as("INSERT INTO users (display_name, safe_username, email, password_hash, confirmed) VALUES ($1, $2, $3, $4, $5) RETURNING id;")
            .bind(display_name)
            .bind(display_name.to_string().to_lowercase())
            .bind(email)
            .bind(password_hash)
            .bind(auto_confirm)
            .fetch_one(pool)
            .await?;

        Ok(id.0)
    }

    pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<UserDTO>> {
        let user = sqlx::query_as::<_, UserDTO>("SELECT * FROM users where email = $1;")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<UserDTO>> {
        let user = sqlx::query_as::<_, UserDTO>("SELECT * FROM users where safe_username = $1;")
            .bind(username.to_string().to_lowercase())
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn is_empty(pool: &SqlitePool) -> Result<bool> {
        let optional_row = sqlx::query("SELECT id FROM users LIMIT 1;")
            .fetch_optional(pool)
            .await?;

        Ok(optional_row.is_none())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::UserRepository;

    #[tokio::test]
    async fn get_and_create() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();

        // make sure the user doesn't exist
        let res = UserRepository::get_user_by_email(&pool, "me@lol.net")
            .await
            .unwrap();
        assert!(res.is_none());

        let res = UserRepository::get_user_by_username(&pool, "lily")
            .await
            .unwrap();
        assert!(res.is_none());

        // create user
        UserRepository::create_user(&pool, "lily", "me@lol.net", "asd123", false)
            .await
            .unwrap();

        // now, make sure the user is there
        let res = UserRepository::get_user_by_email(&pool, "me@lol.net")
            .await
            .unwrap();
        assert!(res.is_some());

        let res = UserRepository::get_user_by_username(&pool, "lily")
            .await
            .unwrap();
        assert!(res.is_some());
    }
}
