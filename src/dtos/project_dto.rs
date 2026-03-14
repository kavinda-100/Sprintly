use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectPayload {
    pub workspace_id: Uuid,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Project name must be between 2 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProjectPayload {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Project name must be between 2 and 100 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
