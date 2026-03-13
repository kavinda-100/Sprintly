use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 12))]
    pub password: String,

    #[validate(length(min = 6, max = 12))]
    pub confirm_password: String,
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
