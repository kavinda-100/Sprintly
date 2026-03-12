use dotenvy::dotenv;
use tower_http::trace::TraceLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tracing_subscriber::EnvFilter;

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

    // Initialize tracing subscriber for logging
    // Default to "debug" level to show TraceLayer logs, override with RUST_LOG env var
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .init();

    // Create a TraceLayer for logging HTTP requests and responses
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));

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
    let app = create_routes()
        .with_state(app_state)
        // Add logging middleware
        .layer(trace);

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
