use dotenvy::dotenv;

mod config;
mod controllers;
mod db;
mod dtos;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use crate::{
    config::{AppState, env::EnvConfig},
    routes::create_routes,
};

#[tokio::main]
async fn main() {
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

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env_config.port))
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:{}", env_config.port);
    println!(
        "API v1 available at http://0.0.0.0:{}/api/v1",
        env_config.port
    );
    axum::serve(listener, app).await.unwrap();
}
