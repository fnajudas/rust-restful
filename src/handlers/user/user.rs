use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value, Map};
use axum::http::StatusCode;
use validator::Validate;

use crate::usecases::user::{find_user_by_id, create_user};
use crate::models::user::UserInput;
use crate::models::response::build_error_response;

pub async fn handling_user_by_id(Path(id): Path<u64>) -> impl IntoResponse {
    match find_user_by_id(id) {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(msg) => {
            // Pengembalian error dengan pesan yang lebih terstruktur
            (StatusCode::NOT_FOUND, Json(json!({ "error": msg })))
        }
    }
}

pub async fn handling_create_user(Json(payload): Json<UserInput>) -> impl IntoResponse {
    // Input Validation
    if let Err(validation_errors) = payload.validate() {
        let mut error_map = Map::new();

        // Membuat map untuk field dan pesan error validasi
        for (field, errors) in validation_errors.field_errors() {
            let messages: Vec<Value> = errors.iter()
                .filter_map(|e| e.message.as_ref())
                .map(|msg| Value::String(msg.to_string()))
                .collect();

            error_map.insert(field.to_string(), Value::Array(messages));
        }

        return (StatusCode::BAD_REQUEST, Json(Value::Object(error_map)));
    }

    // Create User
    match create_user(payload) {
        Ok(message) => {
            // Success response
            let success_response = json!({
                "status": StatusCode::CREATED.as_u16(),
                "message": message,
            });
            (StatusCode::CREATED, Json(success_response))
        }
        Err(err_msg) => {
            // Error response 
            build_error_response(
                StatusCode::BAD_REQUEST,
                "Bad Request",
                &err_msg,
            )
        }
    }
}
