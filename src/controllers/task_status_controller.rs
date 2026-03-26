use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::AppState,
    dtos::task_status_dto::{CreateTaskStatusPayload, TaskStatusResponse},
    middleware::auth::AuthUser,
    models::{Project, TaskStatus},
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Creates a new task status for a project.
 * Validates the input, checks if the project exists, and stores the task status in the database.
 * Route: POST /api/v1/projects/{project_id}/task-statuses
 * access: requires authentication
 */
pub async fn create_task_status(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateTaskStatusPayload>,
) -> Result<Json<ApiResponse<TaskStatusResponse>>, ApiError> {
    // log the incoming request
    tracing::info!(
        "User {} is creating a task status for project {}",
        auth_user.id,
        project_id
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // Check if the project exists,
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching project: {}", e);
            ApiError::InternalServerError("Failed to fetch project".into())
        })?;

    // If the project doesn't exist, return a 404 error
    if project.is_none() {
        tracing::warn!("Project with id {} not found", project_id);
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // create a new task status in the database
    let task_status = sqlx::query_as::<_, TaskStatus>(
        "INSERT INTO task_statuses (project_id, name) VALUES ($1, $2) RETURNING *",
    )
    .bind(project_id)
    .bind(payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Error creating task status: {}", e);
        ApiError::InternalServerError("Failed to create task status".into())
    })?;

    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "Task status created successfully",
        Some(TaskStatusResponse {
            id: task_status.id,
            project_id: task_status.project_id,
            name: task_status.name,
            created_at: task_status.created_at,
            updated_at: task_status.updated_at,
        }),
    )))
}
