use dotenvy::dotenv;
use std::env;


mod utils;
mod config;
mod db;
mod middleware;
mod services;
mod controllers;
mod models;
mod routes;

use crate::{config::AppState, routes::create_routes};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Establish database connection and run migrations
    let pool = db::establish_connection().await;

    // Create application state with the database pool
    let app_state = AppState { db: pool };

    // Build the application with all routes
    let app = create_routes().with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:{}", port);
    println!("API v1 available at http://0.0.0.0:{}/api/v1", port);
    axum::serve(listener, app).await.unwrap();
}
