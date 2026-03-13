use axum::{Json, extract::State, http::StatusCode};

use crate::{
    config::AppState,
    dtos::workspace_dto::{CreateWorkspacePayload, WorkspaceResponse},
    middleware::auth::AuthUser,
    models::Workspace,
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

pub async fn create_workspace(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<Json<ApiResponse<WorkspaceResponse>>, ApiError> {
    // logging the workspace creation attempt with the email (but not the password)
    tracing::info!("Attempting to create workspace with name: {}", payload.name);

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // create the workspace in the database
    let new_workspace = sqlx::query_as::<_, Workspace>(
        "INSERT INTO workspaces (name, owner_id) VALUES ($1, $2) RETURNING *",
    )
    .bind(&payload.name)
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| ApiError::InternalServerError(format!("Failed to create workspace: {}", e)))?;

    // Placeholder implementation
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "Workspace created successfully",
        Some(WorkspaceResponse {
            id: new_workspace.id,
            name: new_workspace.name,
            owner_id: new_workspace.owner_id,
            created_at: new_workspace.created_at,
            updated_at: new_workspace.updated_at,
        }),
    )))
}
