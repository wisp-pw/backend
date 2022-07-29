use crate::{prelude::*, repositories::user::UserRepository};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub enum RegisterError {
    EmailUsed,
    UsernameUsed,
    UnexpectedError,
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
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

    // create user
    // TODO: Hash password
    UserRepository::create_user(
        &state.sql_pool,
        &request.username,
        &request.email,
        &request.password,
    )
    .await
    .map_err(|_| RegisterError::UnexpectedError)
    .map(|_| GenericResponse::ok_msg("CREATED"))
}
