use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::AppState,
    dtos::{
        project_dto::ProjectResponse,
        workspace_dto::{
            CreateWorkspacePayload, UpdateWorkspacePayload, WorkspaceProjectQuery,
            WorkspaceResponse,
        },
    },
    middleware::auth::AuthUser,
    models::{Project, Workspace},
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Creates a new workspace with the provided name for the authenticated user.
 * Validates the input, stores the workspace in the database, and returns a JSON response with the workspace details.
 * Route: POST /api/v1/workspaces
 * access: requires authentication
 * access: any authenticated user can create a workspace, but the workspace will be owned by the user who created it and only that user will have access to modify it.
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
    .map_err(|_| ApiError::InternalServerError("Failed to create workspace".into()))?;

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
 * Route: GET /api/v1/workspaces
 * access: requires authentication
 * access: only workspaces that are owned by the authenticated user will be returned.
 */
pub async fn get_my_workspaces(
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
    .map_err(|_| ApiError::InternalServerError("Failed to retrieve workspaces".into()))?;

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

/**
 * Retrieves the details of a specific workspace by its ID for the user.
 * Validates the workspace ID, checks if the workspace exists and belongs to the user, retrieves it from the database, and returns a JSON response with the workspace details.
 * Route: GET /api/v1/workspaces/{workspace_id}
 * access: requires authentication
 * access: any authenticated user can attempt to retrieve a workspace by ID.
 */
pub async fn get_workspace_by_id(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<ApiResponse<WorkspaceResponse>>, ApiError> {
    // logging the workspace retrieval attempt
    tracing::info!(
        "User {} is attempting to retrieve workspace with id: {}",
        user.email,
        workspace_id
    );

    // get the workspace from the database.
    let workspace = sqlx::query_as::<_, Workspace>("SELECT * FROM workspaces WHERE id = $1")
        .bind(workspace_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to retrieve workspace".into()))?;

    // if the workspace doesn't exist or doesn't belong to the user, return a 404 error
    let workspace = workspace.ok_or_else(|| {
        tracing::warn!(
            "Workspace with id {} not found for user {}",
            workspace_id,
            user.email
        );
        ApiError::NotFound("Workspace not found".into())
    })?;

    // Placeholder implementation
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Workspace retrieved successfully",
        Some(WorkspaceResponse {
            id: workspace.id,
            name: workspace.name,
            owner_id: workspace.owner_id,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
        }),
    )))
}

/**
 * Updates the name of an existing workspace that belongs to the authenticated user.
 * Validates the input, checks if the workspace exists and belongs to the user, updates it in the database, and returns a JSON response with the updated workspace details.
 * Route: PUT /api/v1/workspaces/{workspace_id}
 * access: requires authentication
 * access: only the owner of the workspace can update it.
 */
pub async fn update_workspace(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(workspace_id): Path<Uuid>,
    Json(payload): Json<UpdateWorkspacePayload>,
) -> Result<Json<ApiResponse<WorkspaceResponse>>, ApiError> {
    // logging the workspace update attempt with the email (but not the password)
    tracing::info!(
        "User {} is attempting to update workspace with id: {}",
        user.email,
        workspace_id
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // check if the workspace exists and belongs to the user
    let existing_workspace =
        sqlx::query_as::<_, Workspace>("SELECT * FROM workspaces WHERE id = $1 AND owner_id = $2")
            .bind(workspace_id)
            .bind(user.id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| ApiError::InternalServerError("Failed to retrieve workspace".into()))?;

    // if the workspace doesn't exist or doesn't belong to the user, return a 404 error
    if existing_workspace.is_none() {
        tracing::warn!(
            "Workspace with id {} not found for user {}",
            workspace_id,
            user.email
        );
        return Err(ApiError::NotFound("Workspace not found".into()));
    }

    // update the workspace in the database
    let updated_workspace = sqlx::query_as::<_, Workspace>(
        "UPDATE workspaces SET name = $1, updated_at = NOW() WHERE id = $2 AND owner_id = $3 RETURNING *",
    )
    .bind(&payload.name)
    .bind(workspace_id)
    .bind(user.id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to update workspace".into()))?;

    // Placeholder implementation
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Workspace updated successfully",
        Some(WorkspaceResponse {
            id: updated_workspace.id,
            name: updated_workspace.name,
            owner_id: updated_workspace.owner_id,
            created_at: updated_workspace.created_at,
            updated_at: updated_workspace.updated_at,
        }),
    )))
}

/**
 * Deletes an existing workspace that belongs to the authenticated user.
 * Validates the input, checks if the workspace exists and belongs to the user, deletes it from the database, and returns a JSON response confirming the deletion.
 * Route: DELETE /api/v1/workspaces/{workspace_id}
 * access: requires authentication
 * access: only the owner of the workspace can delete it.
 */
pub async fn delete_workspace(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // logging the workspace update attempt with the email (but not the password)
    tracing::info!(
        "User {} is attempting to delete workspace with id: {}",
        user.email,
        workspace_id
    );

    // check if the workspace exists and belongs to the user
    let existing_workspace =
        sqlx::query_as::<_, Workspace>("SELECT * FROM workspaces WHERE id = $1 AND owner_id = $2")
            .bind(workspace_id)
            .bind(user.id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| ApiError::InternalServerError("Failed to retrieve workspace".into()))?;

    // if the workspace doesn't exist or doesn't belong to the user, return a 404 error
    let existing_workspace = existing_workspace.ok_or_else(|| {
        tracing::warn!(
            "Workspace with id {} not found for user {}",
            workspace_id,
            user.email
        );
        ApiError::NotFound("Workspace not found".into())
    })?;

    // delete the workspace from the database
    sqlx::query("DELETE FROM workspaces WHERE id = $1 AND owner_id = $2")
        .bind(workspace_id)
        .bind(user.id)
        .execute(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to delete workspace".into()))?;

    // Return a success response with no data
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        format!(
            "Workspace named: {} deleted successfully",
            existing_workspace.name
        ),
        None,
    )))
}

/**
 * Retrieves a list of projects that belong to a specific id.
 * Validates the workspace ID, checks if the workspace exists and belongs to the user, retrieves the projects from the database, and returns a JSON response with the list of projects.
 * Route: GET /api/v1/workspaces/{workspace_id}/projects?page=1&page_size=20
 * access: requires authentication
 * access: any authenticated user can attempt to retrieve projects for a workspace.
 */
pub async fn get_workspace_projects(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(workspace_id): Path<Uuid>,
    Query(_query): Query<WorkspaceProjectQuery>,
) -> Result<Json<ApiResponse<Vec<ProjectResponse>>>, ApiError> {
    // logging the project retrieval attempt
    tracing::info!(
        "User {} is attempting to retrieve projects for workspace {}",
        user.email,
        workspace_id
    );

    // check the query parameters has values or assign default values
    let page = _query.page.unwrap_or(1);
    let page_size = _query.page_size.unwrap_or(20);

    // check if the workspace exists and belongs to the user
    let existing_workspace =
        sqlx::query_as::<_, Workspace>("SELECT * FROM workspaces WHERE id = $1")
            .bind(workspace_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| ApiError::InternalServerError("Failed to retrieve workspace".into()))?;

    // if the workspace doesn't exist or doesn't belong to the user, return a 404 error
    if existing_workspace.is_none() {
        tracing::warn!(
            "Workspace with id {} not found for user {}",
            workspace_id,
            user.email
        );
        return Err(ApiError::NotFound("Workspace not found".into()));
    }

    // get the projects for the workspace from the database
    let projects = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE workspace_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
    )
    .bind(workspace_id)
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to retrieve projects".into()))?;

    // map the projects to the response DTOs
    let project_responses: Vec<ProjectResponse> = projects
        .into_iter()
        .map(|p| ProjectResponse {
            id: p.id,
            workspace_id: p.workspace_id,
            name: p.name,
            description: p.description,
            created_at: p.created_at,
            updated_at: p.updated_at,
        })
        .collect();

    // Return the list of projects in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Projects retrieved successfully",
        Some(project_responses),
    )))
}
