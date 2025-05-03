use axum::{Router, routing::{get, post}};
use crate::handlers::user::user::{
    handling_user_by_id,
    handling_create_user,
};

pub fn user_routes() -> Router {
    Router::new()
        .nest("/users", Router::new()
            .route("/:id", get(handling_user_by_id))   // Handle GET /users/:id
            .route("/", post(handling_create_user))    // Handle POST /users
        )
}
