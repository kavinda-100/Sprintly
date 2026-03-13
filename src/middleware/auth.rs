use axum::{extract::FromRequestParts, http::request::Parts};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    config::AppState,
    models::User,
    utils::{api_error::ApiError, jwt::verify_jwt},
};

#[allow(dead_code)]
pub struct AuthUser(pub User);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the database pool from the application state
        let cookies = Cookies::from_request_parts(parts, &state)
            .await
            .map_err(|_| ApiError::Unauthorized("No auth cookie found".into()))?;

        // Get the JWT token from the cookies
        let token_cookie = cookies
            .get("token")
            .ok_or(ApiError::Unauthorized("No auth token found".into()))?;

        // Verify the JWT token and extract the claims
        let claims = verify_jwt(token_cookie.value(), &state.env_config.jwt_secret)
            .map_err(|_| ApiError::Unauthorized("Invalid auth token".into()))?;

        // Extract the user ID from the claims
        let user_id: Uuid = claims.sub;

        // Fetch the user from the database using the user ID
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| ApiError::InternalServerError("Failed to fetch user".into()))?;

        // If the user is not found, return an unauthorized error
        let user = user.ok_or(ApiError::Unauthorized("User not found".into()))?;

        // Return the authenticated user
        Ok(AuthUser(user))
    }
}
