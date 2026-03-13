use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(
        email(message = "Invalid email format"),
        length(min = 1, message = "Email cannot be empty")
    )]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 12,
        message = "Password must be between 6 and 12 characters"
    ))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(
        email(message = "Invalid email format"),
        length(min = 1, message = "Email cannot be empty")
    )]
    pub email: String,

    #[validate(length(
        min = 6,
        max = 12,
        message = "Password must be between 6 and 12 characters"
    ))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthUserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub google_id: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
