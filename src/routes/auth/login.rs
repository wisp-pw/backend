use crate::{prelude::*, repositories::user::UserRepository};

use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub(crate) email_or_username: String,
    pub(crate) password: String,
}

pub enum LoginError {
    InvalidEmail,
    InvalidUsername,
    InvalidPassword,
    UnexpectedError,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            LoginError::InvalidEmail => (StatusCode::BAD_REQUEST, "INVALID_EMAIL"),
            LoginError::InvalidUsername => (StatusCode::BAD_REQUEST, "INVALID_USERNAME"),
            LoginError::InvalidPassword => (StatusCode::BAD_REQUEST, "INVALID_PASSWORD"),
            LoginError::UnexpectedError => (
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
    Json(request): Json<LoginRequest>,
) -> Result<Response, LoginError> {
    let logging_in_with_email = EmailAddress::is_valid(&request.email_or_username);

    debug!("(login) logging_in_with_email = {logging_in_with_email}");

    // get the user
    let user = if logging_in_with_email {
        UserRepository::get_user_by_email(&state.sql_pool, &request.email_or_username)
            .await
            .map_err(handle_err!(LoginError::UnexpectedError))?
            .ok_or(LoginError::InvalidEmail)?
    } else {
        UserRepository::get_user_by_username(&state.sql_pool, &request.email_or_username)
            .await
            .map_err(handle_err!(LoginError::UnexpectedError))?
            .ok_or(LoginError::InvalidUsername)?
    };

    // validate the password
    let password_hash =
        PasswordHash::new(&user.password_hash).map_err(handle_err!(LoginError::UnexpectedError))?;

    Argon2::default()
        .verify_password(request.password.as_bytes(), &password_hash)
        .map_err(handle_err!(LoginError::InvalidPassword))?;

    let access_token = state
        .jwt_service
        .generate_access_token(user)
        .map_err(|_| LoginError::UnexpectedError)?;

    Ok(GenericResponse::ok_msg(&access_token))
}
