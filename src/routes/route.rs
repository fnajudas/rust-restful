use axum::{Router, routing::{get, post}};
use crate::handlers::user::user::{
    handling_user_by_id,
    handling_create_user,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users/:id", get(handling_user_by_id))
        .route("/users", post(handling_create_user))
}
