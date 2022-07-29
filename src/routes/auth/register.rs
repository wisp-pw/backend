use crate::{prelude::*, repositories::user::UserRepository};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub enum RegisterError {
    InvalidEmail,
    EmailUsed,
    UsernameUsed,
    UnexpectedError,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RegisterError::InvalidEmail => (StatusCode::BAD_REQUEST, "INVALID_EMAIL"),
            RegisterError::EmailUsed => (StatusCode::BAD_REQUEST, "EMAIL_USED"),
            RegisterError::UsernameUsed => (StatusCode::BAD_REQUEST, "USERNAME_USED"),
            RegisterError::UnexpectedError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unexpected error has occurred.",
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

pub async fn post(
    Extension(state): Extension<Arc<WispState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Response, RegisterError> {
    // validate email address
    if !EmailAddress::is_valid(&request.email) {
        return Err(RegisterError::InvalidEmail);
    }

    // check email is not in use
    UserRepository::get_user_by_email(&state.sql_pool, &request.email)
        .await
        .map_err(|_| RegisterError::UnexpectedError)?
        .ok_err(RegisterError::EmailUsed)?;

    // check username is not in use
    UserRepository::get_user_by_username(&state.sql_pool, &request.username)
        .await
        .map_err(|_| RegisterError::UnexpectedError)?
        .ok_err(RegisterError::UsernameUsed)?;

    // hash password with Argon2i
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_bytes = request.password.as_bytes();
    let password_hash = argon2
        .hash_password(password_bytes, &salt)
        .map_err(|_| RegisterError::UnexpectedError)?
        .to_string();

    // create user
    UserRepository::create_user(
        &state.sql_pool,
        &request.username,
        &request.email,
        &password_hash,
    )
    .await
    .map_err(|_| RegisterError::UnexpectedError)
    .map(|_| GenericResponse::status_msg(StatusCode::CREATED, "CREATED"))
}
