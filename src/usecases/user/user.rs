use crate::models::user::{User, UserInput};

pub fn find_user_by_id(id: u64) -> Result<User, String> {
    if id == 0 {
        return Err("You do not exist.".into());
    }

    if id.to_string().len() <= 1 {
        return Err("ID must be longer than 1 digit".into());
    }

    if id == 12 {
        if id_is_valid(id) {
            return Ok(User {
                id,
                fullname: "Najuda".into(),
                username: "najuda".into(),
                email: "najuda@email.com".into(),
                password: "secret".into(),
            });
        } else {
            return Err("User not found".into());
        }
    }

    Err("User not found".into())
}

// Fungsi helper private
fn id_is_valid(id: u64) -> bool {
    id == 12 || id.to_string().starts_with('0')
}

pub fn create_user(payload: UserInput) -> Result<String, String> {
    if payload.fullname == "Fatan" {
        return Err("User already exists".into());
    }

    Ok(format!("User {} created successfully", payload.fullname))
}
