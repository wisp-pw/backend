use crate::prelude::*;

pub struct JwtService {}

impl JwtService {
    pub fn generate_access_token(user: UserDTO) -> Result<String> {
        let user_id = user.id;

        Ok("hi".to_string())
    }

    pub fn get_token_data(token: String) -> Result<(Option<i32>, bool)> {
        Ok((None, false))
    }
}
