#[derive(sqlx::FromRow, Debug)]
pub struct UserDTO {
    pub id: i32,
    pub display_name: String,
    pub safe_username: String,
    pub password_hash: String,
    pub confirmed: bool,
}
