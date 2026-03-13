use axum::{Json, extract::State, http::StatusCode};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use validator::Validate;

use crate::{
    config::AppState,
    dtos::auth_dto::{AuthUserResponse, LoginUserDto, RegisterUserDto},
    middleware::auth::AuthUser,
    models::User,
    utils::{
        api_error::ApiError,
        format_validation_errors,
        hash::{hash_password, verify_password},
        jwt::generate_jwt,
        response::ApiResponse,
    },
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
) -> Result<Json<ApiResponse<AuthUserResponse>>, ApiError> {
    // logging the registration attempt with the email (but not the password)
    tracing::info!("Attempting to register user with email: {}", payload.email);

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // comparing password and confirm_password
    if payload.password != payload.confirm_password {
        return Err(ApiError::BadRequest("Passwords do not match".into()));
    }

    // Check if the email is already registered
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch user".into()))?;

    // If the email is already registered, return a conflict error
    if existing_user.is_some() {
        return Err(ApiError::Conflict("User is already registered".into()));
    }

    // Hash the password
    let hashed_password = hash_password(&payload.password)
        .map_err(|_| ApiError::InternalServerError("Failed to hash password".into()))?;

    // Store the user in the database and return the created user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, name, password_hash) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&payload.email)
    .bind(&payload.name)
    .bind(&hashed_password)
    .fetch_one(&state.db)
    .await
    .map_err(|_| ApiError::InternalServerError("Failed to create user".into()))?;

    // Generate a JWT token for the user
    let token = generate_jwt(user.id, &state.env_config.jwt_secret)
        .map_err(|_| ApiError::InternalServerError("Failed to generate JWT token".into()))?;

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

/**
 * Logs in a user with the provided email and password. Validates the input, checks if the user exists, verifies the password, generates a JWT token, and sets it as an HTTP-only cookie.
 * Returns a JSON response with the user's information if login is successful, or an appropriate error message if validation fails, the user does not exist, the password is incorrect, or an internal error occurs.
 * Path: POST /api/v1/auth/login
 */
pub async fn login_user(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginUserDto>,
) -> Result<Json<ApiResponse<AuthUserResponse>>, ApiError> {
    // logging the login attempt with the email (but not the password)
    tracing::info!("Attempting to login user with email: {}", payload.email);

    // Validate the input payload
    payload.validate().map_err(|e| {
        let error_messages = format_validation_errors(&e);
        tracing::error!("Validation errors: {}", error_messages);
        ApiError::BadRequest(error_messages)
    })?;

    // Check if the user exists in the database
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ApiError::InternalServerError("Failed to fetch user".into()))?;

    // If the user does not exist, return an unauthorized error
    let user = match user {
        Some(user) => user,
        None => {
            return Err(ApiError::NotFound("User does not exist".into()));
        }
    };

    // check if the user has a google_id, if so, they should not be able to login with email and password
    if user.google_id.is_some() {
        return Err(ApiError::Unauthorized("Please login with google".into()));
    }

    // check if the user has a password_hash, if not, they should not be able to login with email and password
    let password_hash = match user.password_hash {
        Some(ref hash) => hash,
        None => {
            tracing::warn!(
                "User with email {} does not have a password hash",
                payload.email
            );
            return Err(ApiError::NotFound("Password not found".into()));
        }
    };

    // Verify the password
    let password_valid = verify_password(&payload.password, password_hash);
    // If the password is incorrect, return an unauthorized error
    if !password_valid {
        return Err(ApiError::Unauthorized("Invalid Credentials".into()));
    }

    // Generate a JWT token for the user
    let token = generate_jwt(user.id, &state.env_config.jwt_secret)
        .map_err(|_| ApiError::InternalServerError("Failed to generate JWT token".into()))?;

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
        StatusCode::OK,
        "User logged in successfully",
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

/**
 * Logs out the current user by removing the JWT token cookie. Returns a JSON response indicating that the user has been logged out successfully.
 * Path: POST /api/v1/auth/logout
 */
pub async fn logout_user(cookies: Cookies) -> Result<Json<ApiResponse<()>>, ApiError> {
    // logging the logout attempt
    tracing::info!("Attempting to logout user");

    // Remove the JWT token cookie by setting it with an expired date
    cookies.add(
        Cookie::build(("token", ""))
            .path("/")
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Lax)
            .build(),
    );

    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "User logged out successfully",
        None,
    )))
}

/**
 * Retrieves the authenticated user's information. Requires the user to be authenticated with a valid JWT token in the cookies. Returns a JSON response with the user's information if authentication is successful, or an appropriate error message if authentication fails.
 * Path: GET /api/v1/auth/me
 */
pub async fn get_me(
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<AuthUserResponse>>, ApiError> {
    Ok(Json(ApiResponse::new(
        true,
        StatusCode::OK,
        "Authenticated user",
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
