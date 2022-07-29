use crate::prelude::*;
use sqlx::SqlitePool;

pub struct UserRepository {}

impl UserRepository {
    pub async fn create_user(
        pool: &SqlitePool,
        display_name: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query("INSERT INTO users (display_name, safe_username, email, password_hash) VALUES ($1, $2, $3, $4)")
            .bind(display_name)
            .bind(display_name.to_string().to_lowercase())
            .bind(email)
            .bind(password_hash)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users where email = $1;")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users where safe_username = $1;")
            .bind(username.to_string().to_lowercase())
            .fetch_optional(pool)
            .await?;

        Ok(user)
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
        assert_eq!(res.is_none(), true);

        let res = UserRepository::get_user_by_username(&pool, "lily")
            .await
            .unwrap();
        assert_eq!(res.is_none(), true);

        // create user
        UserRepository::create_user(&pool, "lily", "me@lol.net", "asd123")
            .await
            .unwrap();

        // now, make sure the user is there
        let res = UserRepository::get_user_by_email(&pool, "me@lol.net")
            .await
            .unwrap();
        assert_eq!(res.is_none(), false);

        let res = UserRepository::get_user_by_username(&pool, "lily")
            .await
            .unwrap();
        assert_eq!(res.is_none(), false);
    }
}
