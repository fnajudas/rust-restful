use serde::Serialize;
use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}

pub fn build_error_response(
    status: StatusCode,
    error: &str,
    message: &str,
) -> (StatusCode, Json<Value>) {
    let err_response = json!({
        "status": status.as_u16(),
        "error": error,
        "message": message,
    });
    (status, Json(err_response))
}
