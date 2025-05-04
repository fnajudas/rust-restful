use serde::Serialize;
use axum::{http::StatusCode, Json, response::IntoResponse};
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status: u16,
    pub message: String,
    pub data: Value,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}

/// Success response builder
pub fn build_success_response(
    status: StatusCode,
    message: &str,
    data: Value,
) -> impl IntoResponse {
    let response = SuccessResponse {
        status: status.as_u16(),
        message: message.to_string(),
        data,
    };

    (status, Json(response))
}

/// Error response builder
pub fn build_error_response(
    status: StatusCode,
    error: &str,
    message: &str,
) -> impl IntoResponse {
    let err_response = ErrorResponse {
        status: status.as_u16(),
        error: error.to_string(),
        message: message.to_string(),
    };

    (status, Json(err_response))
}
