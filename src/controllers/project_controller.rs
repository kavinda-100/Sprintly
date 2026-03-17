use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::AppState,
    dtos::{
        project_dto::{CreateProjectPayload, ProjectResponse, UpdateProjectPayload},
        task_dto::TaskResponse,
    },
    middleware::auth::AuthUser,
    models::{Project, Task},
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Controller function to create a new project within a workspace. Validates the input payload, checks user authentication, and interacts with the database to persist the new project. Returns a structured API response with the created project details or appropriate error messages.
 * Path: POST /api/v1/projects
 */
pub async fn create_project(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<ApiResponse<ProjectResponse>>, ApiError> {
    // logging the project creation attempt with the email (but not the password)
    tracing::info!(
        "Attempting to create project with name: {} for workspace: {} by user: {}",
        payload.name,
        payload.workspace_id,
        user.email
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // create the project in the database
    let project = sqlx::query_as::<_, Project>(
        "INSERT INTO projects (workspace_id, name, description)
        VALUES ($1, $2, $3)
        RETURNING *",
    )
    .bind(payload.workspace_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to create project".into()))?;

    // return the created project in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "Project created successfully",
        Some(ProjectResponse {
            id: project.id,
            workspace_id: project.workspace_id,
            name: project.name,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }),
    )))
}

/**
 * Controller function to update an existing project. Validates the input payload, checks user authentication, verifies project existence, and updates the project details in the database. Returns a structured API response with the updated project details or appropriate error messages.
 * Path: PUT /api/v1/projects/{id}
 */
pub async fn update_project(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<UpdateProjectPayload>,
) -> Result<Json<ApiResponse<ProjectResponse>>, ApiError> {
    // logging the project update attempt with the email (but not the password)
    tracing::info!(
        "Attempting to update project with id: {} by user: {}",
        project_id,
        user.email
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // Check if the project exists
    let existing_project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to query project".into()))?;

    // If the project does not exist, return a 404 error
    if existing_project.is_none() {
        tracing::warn!("Project with id: {} not found", project_id);
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // Update the project in the database
    let updated_project = sqlx::query_as::<_, Project>(
        "UPDATE projects SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = NOW()
        WHERE id = $3
        RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(project_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to update project".into()))?;

    // return the updated project in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Project updated successfully",
        Some(ProjectResponse {
            id: updated_project.id,
            workspace_id: updated_project.workspace_id,
            name: updated_project.name,
            description: updated_project.description,
            created_at: updated_project.created_at,
            updated_at: updated_project.updated_at,
        }),
    )))
}

/**
 * Controller function to delete an existing project. Checks user authentication, verifies project existence, and deletes the project from the database. Returns a structured API response indicating success or appropriate error messages.
 * Path: DELETE /api/v1/projects/{id}
 */
pub async fn delete_project(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // logging the project deletion attempt with the email (but not the password)
    tracing::info!(
        "Attempting to delete project with id: {} by user: {}",
        project_id,
        user.email
    );

    // Check if the project exists
    let existing_project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to query project".into()))?;

    // If the project does not exist, return a 404 error
    if existing_project.is_none() {
        tracing::warn!("Project with id: {} not found", project_id);
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // Delete the project from the database
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(project_id)
        .execute(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to delete project".into()))?;

    // return a success response with no data
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Project deleted successfully",
        None,
    )))
}

/**
 * Controller function to fetch all tasks for a specific project. Checks user authentication, verifies project existence, and retrieves all tasks associated with the project from the database. Returns a structured API response with a list of tasks or appropriate error messages.
 * Path: GET /api/v1/projects/{project_id}/tasks
 */
pub async fn get_all_tasks_for_project(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TaskResponse>>>, ApiError> {
    // logging the task retrieval attempt with the email (but not the password)
    tracing::info!(
        "Attempting to fetch tasks for project: {} by user: {}",
        project_id,
        user.email
    );

    // Check if the project exists
    let existing_project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to query project".into()))?;

    // If the project does not exist, return a 404 error
    if existing_project.is_none() {
        tracing::warn!("Project with id: {} not found", project_id);
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // Fetch all tasks for the project
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to query tasks".into()))?;

    // Convert tasks to TaskResponse
    let task_responses: Vec<TaskResponse> = tasks
        .into_iter()
        .map(|task| TaskResponse {
            id: task.id,
            project_id: task.project_id,
            title: task.title,
            description: task.description,
            task_status: task.task_status,
            task_priority: task.task_priority,
            owner_id: task.owner_id,
            due_date: task.due_date,
            position: task.position,
            created_at: task.created_at,
            updated_at: task.updated_at,
        })
        .collect();

    // Return the list of tasks in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Tasks fetched successfully",
        Some(task_responses),
    )))
}
