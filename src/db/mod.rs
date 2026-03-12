use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

/// Establishes a connection to the PostgreSQL database
pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    //Run database migrations to ensure the tables exist
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}
