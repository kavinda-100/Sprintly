use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use crate::{
    config::AppState,
    dtos::workspace_dto::{CreateWorkspacePayload, WorkspaceResponse},
    middleware::auth::AuthUser,
    models::Workspace,
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Creates a new workspace with the provided name for the authenticated user.
 * Validates the input, stores the workspace in the database, and returns a JSON response with the workspace details.
 * Route: POST /api/v1/workspaces
 */
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

/**
 * Retrieves a list of workspaces that the authenticated user has access to.
 * This is a placeholder implementation that currently returns an empty list.
 * Route: GET /api/v1/workspaces
 */
pub async fn get_workspaces(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<Vec<WorkspaceResponse>>>, ApiError> {
    // logging the workspace retrieval attempt
    tracing::info!("User {} is attempting to retrieve workspaces", user.email);

    // get the workspaces from the database that the user has access to (owned)
    let _workspaces = sqlx::query_as::<_, Workspace>(
        "SELECT * FROM workspaces WHERE owner_id = $1 ORDER BY created_at DESC",
    )
    .bind(user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalServerError(format!("Failed to retrieve workspaces: {}", e)))?;

    // map the workspaces to the response DTOs
    let _workspace_responses: Vec<WorkspaceResponse> = _workspaces
        .into_iter()
        .map(|ws| WorkspaceResponse {
            id: ws.id,
            name: ws.name,
            owner_id: ws.owner_id,
            created_at: ws.created_at,
            updated_at: ws.updated_at,
        })
        .collect();

    // Placeholder implementation
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Workspaces retrieved successfully",
        Some(_workspace_responses), // Return the list of workspaces
    )))
}
