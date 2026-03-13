use axum::{Json, extract::State, http::StatusCode};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use validator::Validate;

use crate::{
    config::AppState,
    dtos::auth_dto::{AuthUserResponse, LoginUserDto, RegisterUserDto},
    models::User,
    utils::{
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

/**
 * Logs in a user with the provided email and password. Validates the input, checks if the user exists, verifies the password, generates a JWT token, and sets it as an HTTP-only cookie.
 * Returns a JSON response with the user's information if login is successful, or an appropriate error message if validation fails, the user does not exist, the password is incorrect, or an internal error occurs.
 * Path: POST /api/v1/auth/login
 */
pub async fn login_user(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginUserDto>,
) -> Result<Json<ApiResponse<AuthUserResponse>>, StatusCode> {
    // logging the login attempt with the email (but not the password)
    tracing::info!("Attempting to login user with email: {}", payload.email);

    // Validate the input payload
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    // Check if the user exists in the database
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the user does not exist, return an unauthorized error
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(Json(ApiResponse::new(
                false,
                StatusCode::UNAUTHORIZED,
                "User does not exist",
                None,
            )));
        }
    };

    // check if the user has a google_id, if so, they should not be able to login with email and password
    if user.google_id.is_some() {
        return Ok(Json(ApiResponse::new(
            false,
            StatusCode::UNAUTHORIZED,
            "Please login with Google",
            None,
        )));
    }

    // check if the user has a password_hash, if not, they should not be able to login with email and password
    let password_hash = match user.password_hash {
        Some(ref hash) => hash,
        None => {
            tracing::warn!(
                "User with email {} does not have a password hash",
                payload.email
            );
            return Ok(Json(ApiResponse::new(
                false,
                StatusCode::UNAUTHORIZED,
                "Invalid Credentials",
                None,
            )));
        }
    };

    // Verify the password
    let password_valid = verify_password(&payload.password, password_hash);
    // If the password is incorrect, return an unauthorized error
    if !password_valid {
        return Ok(Json(ApiResponse::new(
            false,
            StatusCode::UNAUTHORIZED,
            "Invalid Credentials",
            None,
        )));
    }

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
pub async fn logout_user(cookies: Cookies) -> Result<Json<ApiResponse<()>>, StatusCode> {
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
