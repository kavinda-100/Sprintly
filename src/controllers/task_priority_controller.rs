use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::AppState,
    dtos::task_priority_dto::{CreateTaskPriorityPayload, TaskPriorityResponse},
    middleware::auth::AuthUser,
    models::{Project, TaskPriority},
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Creates a new task priority for a project.
 * Validates the input, checks if the project exists, and stores the task priority in the database.
 * Route: POST /api/v1/projects/{project_id}/task-priorities
 * access: requires authentication
 */
pub async fn create_task_priority(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateTaskPriorityPayload>,
) -> Result<Json<ApiResponse<TaskPriorityResponse>>, ApiError> {
    // log the incoming request
    tracing::info!(
        "User {} is creating a task priority for project {}",
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

    // create a new task priority in the database
    let task_priority = sqlx::query_as::<_, TaskPriority>(
        "INSERT INTO task_priorities (project_id, name) VALUES ($1, $2) RETURNING *",
    )
    .bind(project_id)
    .bind(payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Error creating task priority: {}", e);
        ApiError::InternalServerError("Failed to create task priority".into())
    })?;

    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "Task priority created successfully",
        Some(TaskPriorityResponse {
            id: task_priority.id,
            project_id: task_priority.project_id,
            name: task_priority.name,
            created_at: task_priority.created_at,
            updated_at: task_priority.updated_at,
        }),
    )))
}
