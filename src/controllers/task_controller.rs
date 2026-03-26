use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    config::AppState,
    dtos::task_dto::{CreateTaskPayload, TaskResponse, UpdateTaskPayload},
    middleware::auth::AuthUser,
    models::{Project, Task},
    utils::{api_error::ApiError, format_validation_errors, response::ApiResponse},
};

/**
 * Handler for creating a new task within a project. Validates the input payload, checks user authentication, and inserts the new task into the database. Returns the created task in the response if successful.
 * Path: POST /api/v1/tasks
 * access: requires authentication
 * access: any authenticated user can create a task within a project they have access to.
 */
pub async fn create_task(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    // logging the task creation attempt with the email (but not the password)
    tracing::info!(
        "Attempting to create task with title: {} for project: {} by user: {}",
        payload.title,
        payload.project_id,
        user.email
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // check if the project exists,
    let project_exists = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(payload.project_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch project".into()))?;

    if project_exists.is_none() {
        return Err(ApiError::NotFound("Project not found".into()));
    }

    // create the task in the database
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (project_id, title, description, task_status, task_priority, owner_id, due_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *",
    )
    .bind(payload.project_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.task_status)
    .bind(payload.task_priority)
    .bind(user.id)
    .bind(payload.due_date)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create task: {:?}", e);
        ApiError::InternalServerError("Failed to create task".into())
    })?;

    // return the created task in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "Task created successfully",
        Some(TaskResponse {
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
        }),
    )))
}

/**
 * Handler for updating an existing task. Validates the input payload, checks user authentication, and updates the task in the database if it exists and belongs to the user. Returns the updated task in the response if successful.
 * Path: PUT /api/v1/tasks/{task_id}
 * access: requires authentication
 * access: any authenticated user can update a task they own.
 */
pub async fn update_task(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    // logging the task update attempt with the email (but not the password)
    tracing::info!(
        "Attempting to update task: {} by user: {}",
        task_id,
        user.email
    );

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // check if the task exists and belongs to the user (no need to owner of the task)
    let existing_task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch task".into()))?;

    // if the task doesn't exist, or user is not the owner, return a 404 error
    if existing_task.is_none() {
        tracing::warn!("Task not found or does not belong to user: {}", task_id);
        return Err(ApiError::NotFound("Task not found".into()));
    }

    // update the task in the database
    let updated_task = sqlx::query_as::<_, Task>(
        "UPDATE tasks SET title = COALESCE($1, title), description = COALESCE($2, description), 
            task_status = COALESCE($3, task_status), task_priority = COALESCE($4, task_priority), 
            due_date = COALESCE($5, due_date), position = COALESCE($6, position), updated_at = NOW() 
            WHERE id = $7 RETURNING *",
    )
    .bind(payload.title)
    .bind(payload.description)
    .bind(payload.task_status)
    .bind(payload.task_priority)
    .bind(payload.due_date)
    .bind(payload.position)
    .bind(task_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to update task".into()))?;

    // return the updated task in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Task updated successfully",
        Some(TaskResponse {
            id: updated_task.id,
            project_id: updated_task.project_id,
            title: updated_task.title,
            description: updated_task.description,
            task_status: updated_task.task_status,
            task_priority: updated_task.task_priority,
            owner_id: updated_task.owner_id,
            due_date: updated_task.due_date,
            position: updated_task.position,
            created_at: updated_task.created_at,
            updated_at: updated_task.updated_at,
        }),
    )))
}

/**
 * Handler for deleting an existing task. Checks user authentication and deletes the task from the database if it exists and belongs to the user. Returns a success message in the response if successful.
 * Path: DELETE /api/v1/tasks/{task_id}
 * access: requires authentication
 * access: any authenticated user can delete a task they own.
 */
pub async fn delete_task(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // logging the task deletion attempt with the email (but not the password)
    tracing::info!(
        "Attempting to delete task: {} by user: {}",
        task_id,
        user.email
    );

    // check if the task exists and belongs to the user (no need to owner of the task)
    let existing_task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch task".into()))?;

    // if the task doesn't exist, or user is not the owner, return a 404 error
    let task = match existing_task {
        Some(task) => task,
        None => {
            tracing::warn!("Task not found or does not belong to user: {}", task_id);
            return Err(ApiError::NotFound("Task not found".into()));
        }
    };

    // only the owner of the task can delete it
    if task.owner_id != user.id {
        tracing::warn!("User {} is not the owner of task: {}", user.email, task_id);
        return Err(ApiError::Forbidden(
            "You do not have permission to delete this task".into(),
        ));
    }

    // delete the task from the database
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(task_id)
        .execute(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to delete task".into()))?;

    // return a success response with no data
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Task deleted successfully",
        None,
    )))
}

/**
 * Handler for retrieving a task by its ID. Checks user authentication and fetches the task from the database if it exists and belongs to the user. Returns the retrieved task in the response if successful.
 * Path: GET /api/v1/tasks/{task_id}
 * access: requires authentication
 * access: any authenticated user can retrieve a task they have access to.
 */
pub async fn get_task_by_id(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<TaskResponse>>, ApiError> {
    // logging the task retrieval attempt with the email (but not the password)
    tracing::info!(
        "Attempting to retrieve task: {} by user: {}",
        task_id,
        user.email
    );

    // fetch the task from the database
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch task".into()))?;

    // if the task doesn't exist, return a 404 error
    let task = match task {
        Some(task) => task,
        None => {
            tracing::warn!("Task not found: {}", task_id);
            return Err(ApiError::NotFound("Task not found".into()));
        }
    };

    // return the retrieved task in the response
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Task retrieved successfully",
        Some(TaskResponse {
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
        }),
    )))
}
