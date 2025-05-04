use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use serde_json::{json, to_value};
use axum::http::StatusCode;

use crate::usecases::user::{find_user_by_id, create_user};
use crate::models::user::UserInput;
use crate::models::response::{build_error_response, build_success_response};

pub async fn handling_user_by_id(Path(id): Path<u64>) -> impl IntoResponse {
    match find_user_by_id(id) {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(msg) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": msg })))
        }
    }
}

pub async fn handling_create_user(Json(payload): Json<UserInput>) -> impl IntoResponse {
    match create_user(payload.clone()) {
        Ok(_) => {
            let datas = to_value(payload).unwrap_or_default();
            build_success_response(StatusCode::CREATED, "User created successfully", datas).into_response()
        }
        Err(err_msg) => {
            build_error_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", &err_msg).into_response()
        }
    }
}
