use axum::{Json, extract::State, http::StatusCode};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use validator::Validate;

use crate::{
    config::AppState,
    dtos::auth_dto::{AuthUserResponse, RegisterUserDto},
    models::User,
    utils::{hash::hash_password, jwt::generate_jwt, response::ApiResponse},
};

/**
 * Registers a new user with the provided email, name, password, and confirm_password.
 * Validates the input, hashes the password, stores the user in the database, generates a JWT token, and sets it as an HTTP-only cookie.
 * Returns a JSON response with the user's information if registration is successful, or an appropriate error message if validation fails or an internal error occurs.
 * Path: POST /api/v1/auth/register
 */
pub async fn register_user(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<RegisterUserDto>,
) -> Result<Json<ApiResponse<AuthUserResponse>>, StatusCode> {
    // logging the registration attempt with the email (but not the password)
    tracing::info!("Attempting to register user with email: {}", payload.email);

    // Validate the input payload
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    // comparing password and confirm_password
    if payload.password != payload.confirm_password {
        return Ok(Json(ApiResponse::new(
            false,
            StatusCode::BAD_REQUEST,
            "Passwords do not match",
            None,
        )));
    }

    // Check if the email is already registered
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the email is already registered, return a conflict error
    if existing_user.is_some() {
        return Ok(Json(ApiResponse::new(
            false,
            StatusCode::CONFLICT,
            "User is already registered",
            None,
        )));
    }

    // Hash the password
    let hashed_password =
        hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store the user in the database and return the created user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, name, password_hash) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.email)
    .bind(&payload.name)
    .bind(&hashed_password)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate a JWT token for the user
    let token = generate_jwt(user.id, &state.env_config.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set the JWT token as an HTTP-only cookie
    cookies.add(
        Cookie::build(("token", token))
            .path("/")
            .http_only(true)
            .secure(state.env_config.dev_mode != "development")
            .same_site(SameSite::Lax)
            .build(),
    );

    // Return a JSON response with the user's information
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::CREATED,
        "User registered successfully",
        Some(AuthUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            google_id: user.google_id,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }),
    )))
}
