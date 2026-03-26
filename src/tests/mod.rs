use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    response::Response,
};
use dotenvy::dotenv;
use tower::ServiceExt;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

#[cfg(test)]
mod auth_test;
#[cfg(test)]
mod health_test;
#[cfg(test)]
mod workspace_test;

use crate::{
    config::{AppState, env::EnvConfig},
    db,
    models::User,
    routes::create_routes,
};

/**
 * This function create a test user for testing. It returns a User struct with predefined values. This user can be used in tests to simulate an authenticated user without going through the actual registration and login process. The function generates a new UUID for the user ID, sets a fixed email and name, and leaves optional fields as None. The created_at and updated_at fields are set to the current time. This allows tests to focus on the functionality being tested without worrying about user creation and authentication. It avoid auth middleware by directly creating a test user and using the test mode of
 * the application which bypasses authentication and returns the test user for any request that requires authentication
 */
pub fn create_test_user() -> User {
    User {
        id: "1f29c101-830e-4d99-80a0-48b8b56fa091"
            .parse::<Uuid>()
            .unwrap(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        google_id: None,
        avatar_url: None,
        password_hash: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    }
}

/**
 * This function is called before each test to set up the application state and return the application router. It loads environment variables, establishes a database connection, and builds the application with all routes. Each test can call this function to get a fresh instance of the application with a clean state.
 * Returns a Router that can be used to send test requests to the application.
 */
#[allow(dead_code)]
#[allow(warnings)]
pub async fn before_each_test() -> Router {
    // Load environment variables from .env file
    dotenv().ok();

    // Load environment configuration
    let env_config = EnvConfig::from_env();

    // Establish database connection and run migrations
    let pool = db::establish_connection(&env_config.database_url).await;

    // Create application state with the database pool
    let app_state = AppState {
        app_name: String::from("Sprintly API"), // app name
        db: pool,                               // pass the database pool to the application state
        env_config: env_config.clone(), // pass the environment configuration to the application state
    };

    // Build the application with all routes
    let app = create_routes(app_state)
        // Add cookie management middleware
        .layer(CookieManagerLayer::new());

    app
}

/**
 * This function is a helper to send a request to the application and get the response. It takes the application router and a request, sends the request to the application, and returns the response. This is useful for testing individual endpoints by simulating HTTP requests and checking the responses.
 */
#[allow(dead_code)]
pub async fn send_request(app: Router, request: Request<Body>) -> Response {
    app.clone().into_service().oneshot(request).await.unwrap()
}

/**
 * This function is a helper to convert a response body into a string. It takes a Response with a Body, converts the body into bytes, and then converts the bytes into a UTF-8 string. This is useful for testing to easily read and assert the contents of the response body.
 */
#[allow(dead_code)]
pub async fn convert_response_to_string(response: Response<Body>) -> String {
    // Convert the response body into bytes
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    // Print status and body before assertions so failures still show debug info.
    let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();

    println!("----------------------------------------");
    println!("Response body: {}", body_string);
    println!("----------------------------------------");

    body_string
}
