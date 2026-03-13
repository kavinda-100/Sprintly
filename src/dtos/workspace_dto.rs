use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkspacePayload {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Workspace name must be between 2 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkspacePayload {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Workspace name must be between 2 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
