use crate::{
    prelude::*,
    repositories::{email_confirmations::EmailConfirmationRepository, user::UserRepository},
    services::email::EmailService,
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
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
    Extension(settings): Extension<Arc<WispSettings>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Response, RegisterError> {
    // validate email address
    if !EmailAddress::is_valid(&request.email) {
        return Err(RegisterError::InvalidEmail);
    }

    // check email is not in use
    UserRepository::get_user_by_email(&state.sql_pool, &request.email)
        .await
        .handle_err(RegisterError::UnexpectedError)?
        .ok_err(RegisterError::EmailUsed)?;

    // check username is not in use
    UserRepository::get_user_by_username(&state.sql_pool, &request.username)
        .await
        .handle_err(RegisterError::UnexpectedError)?
        .ok_err(RegisterError::UsernameUsed)?;

    // hash password with Argon2i
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_bytes = request.password.as_bytes();
    let password_hash = argon2
        .hash_password(password_bytes, &salt)
        .handle_err(RegisterError::UnexpectedError)?
        .to_string();

    // if there are no users auto confirm the first user
    let auto_confirm = UserRepository::is_empty(&state.sql_pool)
        .await
        .handle_err(RegisterError::UnexpectedError)?;

    // create user
    let user_id = UserRepository::create_user(
        &state.sql_pool,
        &request.username,
        &request.email,
        &password_hash,
        auto_confirm,
    )
    .await
    .handle_err(RegisterError::UnexpectedError)?;

    // send confirmation email if we arent automatically confirming the user
    if !auto_confirm && settings.email_enabled {
        let code = EmailConfirmationRepository::create(&state.sql_pool, user_id)
            .await
            .handle_err(RegisterError::UnexpectedError)?;

        EmailService::send_confirmation_email(&settings, &request.email, &code)
            .await
            .handle_err(RegisterError::UnexpectedError)?;
    }

    Ok(GenericResponse::status_msg(StatusCode::CREATED, "CREATED"))
}
