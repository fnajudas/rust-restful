use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct UserInput {
    #[validate(length(min=3, message="Fullname must be at least 3 characters"))]
    pub fullname: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Email is not valid"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}