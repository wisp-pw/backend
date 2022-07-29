use crate::prelude::*;

pub struct GenericResponse {}

#[allow(dead_code)]
impl GenericResponse {
    pub fn status_msg(code: StatusCode, msg: &str) -> Response {
        return (code, Json(json!({ "message": msg }))).into_response();
    }

    pub fn ok_msg(msg: &str) -> Response {
        return (StatusCode::OK, Json(json!({ "message": msg }))).into_response();
    }

    pub fn bad_req(body: impl IntoResponse) -> Response {
        return (StatusCode::BAD_REQUEST, body).into_response();
    }

    pub fn bad_req_error_msg(msg: &str) -> Response {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response();
    }

    pub fn bad_req_nb() -> Response {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    pub fn internal_error() -> Response {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
}
