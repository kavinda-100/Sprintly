use axum::Router;
use dotenvy::dotenv;
use uuid::Uuid;

pub mod health_test;

use crate::{
    config::{AppState, env::EnvConfig},
    db,
    models::User,
    routes::create_routes,
};

// This function create a test user for testing
pub fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
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
    let app = create_routes().with_state(app_state);

    app
}
