use crate::prelude::*;

pub struct GenericResponse {}

#[allow(dead_code)]
impl GenericResponse {
    pub fn status_msg(code: StatusCode, msg: &str) -> Response {
        (code, Json(json!({ "message": msg }))).into_response()
    }

    pub fn ok_msg(msg: &str) -> Response {
        (StatusCode::OK, Json(json!({ "message": msg }))).into_response()
    }

    pub fn bad_req(body: impl IntoResponse) -> Response {
        (StatusCode::BAD_REQUEST, body).into_response()
    }

    pub fn bad_req_error_msg(msg: &str) -> Response {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response()
    }

    pub fn bad_req_nb() -> Response {
        (StatusCode::BAD_REQUEST).into_response()
    }

    pub fn internal_error() -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
