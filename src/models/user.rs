#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub safe_username: String,
    pub password_hash: String,
}

impl User {

}