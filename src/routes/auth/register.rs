use crate::{prelude::*, repositories::user::UserRepository};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

pub async fn post(Extension(state): Extension<Arc<WispState>>, Json(request): Json<RegisterRequest>) -> Response {
    // check email is not in use
    match UserRepository::get_user_by_email(&state.sql_pool, &request.email).await {
        Ok(Some(_)) => return GenericResponse::bad_req_error_msg("EMAIL_USED"),
        Err(_) => return GenericResponse::internal_error(),
        _ => {}
    }

    // check username is not in use
    match UserRepository::get_user_by_username(&state.sql_pool, &request.username).await {
        Ok(Some(_)) => return GenericResponse::bad_req_error_msg("USERNAME_USED"),
        Err(_) => return GenericResponse::internal_error(),
        _ => {}
    }

    // create user
    // TODO: Hash password
    match UserRepository::create_user(&state.sql_pool, &request.username, &request.email, &request.password).await {
        Err(_) => return GenericResponse::internal_error(),
        _ => {}
    }

    GenericResponse::ok_msg("CREATED")
}
