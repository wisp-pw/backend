use jwts::{
    jws::{Algorithm, Key, Token},
    Error,
};

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct WispClaims {
    iss: String,
    uid: i32,
}

pub struct JwtService {
    jwt_secret: String,
}

impl JwtService {
    pub fn new(jwt_secret: &str) -> Self {
        JwtService {
            jwt_secret: jwt_secret.to_owned(),
        }
    }

    pub fn generate_access_token(&self, user: UserDTO) -> Result<String, Error> {
        let claims = WispClaims {
            iss: "wisp".to_string(),
            uid: user.id,
        };

        let mut token = Token::with_payload(claims);

        let key = Key::new(self.jwt_secret.as_bytes(), Algorithm::HS512);
        token.sign(&key)
    }

    pub fn _get_token_data(&self, _token: String) -> Result<(Option<i32>, bool)> {
        Ok((None, false))
    }
}
