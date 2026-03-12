# Copilot Instructions for Sprintly API

## Project Overview

Sprintly is a **task management API** built with Rust.  
It uses the following stack:

- **Rust** (latest stable)
- **Axum** web framework
- **Tokio** async runtime
- **SQLx** for PostgreSQL
- **Serde** for JSON serialization
- **dotenv** for environment configuration
- **tower-http** for static file serving

The API follows a **clean architecture** inspired by Express.js MVC:

```
src/
├── main.rs
├── db/
│ └── mod.rs # Database connection & migrations
├── models/ # Data structures and payloads
├── controllers/ # Business logic per resource
├── routes/ # API route composition
├── services/ # External integrations (e.g., email, notifications)
├── middleware/ # Custom middleware
├── DTOs/ # Data Transfer Objects for request/response validation
└── utils/ # Helpers, e.g., API response wrappers
public/ # Static files (HTML, CSS, JS)
API.http # endpoint collection for testing
makefile # Build and run commands
```

---

## General Copilot Guidelines

1. **Rust Best Practices**
    - Use `async/await` for all I/O operations.
    - Use `sqlx::query_as` with compile-time type checking for database queries.
    - Prefer **`UUID` for primary keys** on users and tasks, projects, workspaces,for lookup tables.
    - Use `chrono::Utc` or `chrono::NaiveDateTime` for timestamps.
    - Leverage `Result<T, E>` for error handling; map errors to HTTP status codes.
    - Use **structured logging** where appropriate.

2. **Axum & Routing**
    - Use `Router::new()` and `.route()` for endpoints.
    - Pass shared state via `axum::extract::State`.
    - Serve static files using `tower_http::services::ServeDir` for `/` route and public assets.

3. **Code Style / Personal Preferences**
    - Keep function names snake_case.
    - Keep struct names PascalCase.
    - Organize code into **models → controllers → routes** per resource.
    - Keep a consistent `ApiResponse<T>` wrapper for all JSON responses:

```rust
pub struct ApiResponse<T> {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
}
```

- Use #[derive(Serialize, Deserialize, Debug, FromRow)] where applicable.
- Keep SQL queries readable and aligned, preferably multiline strings for complex queries.
- For task assignments and many-to-many relationships, use join tables (task_assignees) with proper foreign key constraints.

---

## Database Guidelines

- Use SQLx migrations stored in migrations/.
- Prefer foreign keys with ON DELETE CASCADE where appropriate.
- Ensure unique constraints where needed (email, task VIN, etc.).
- Use uuid-ossp extension for UUID generation.

---

## Generating Code with Copilot

- When generating code, follow these rules:
- Generate Rust code only, do not generate pseudo-code or JavaScript.
- Use the project structure: models → controllers → routes.
- Always use async functions for database or HTTP handling.
- Respect existing coding style:
- Indent with 4 spaces
- Use Rust standard formatting (rustfmt)
- Keep imports clean, grouped by standard / external / internal
- When adding routes, always return ApiResponse<T> with consistent keys.
- Avoid cloning large objects unnecessarily; prefer references or Arc<PgPool> for database pool.
- For error handling, map SQLx errors to HTTP status codes (404 for not found, 500 for internal error).

---

## Example: Adding a New Resource

- Create migration in `migrations/`.
- Add model in `src/models/`.
- Add controller in `src/controllers/`.
- Add routes in `src/routes/`.
- Merge routes in `src/routes/api.rs`.
- Test endpoints with `API.http` or `Postman`.
