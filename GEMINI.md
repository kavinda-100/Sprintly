---
description: 'Use when a task spans both backend and frontend in Sprintly (route/DTO/response contract changes). Enforces strict cross-stack compatibility between Rust API and public UI.'
name: 'Sprintly Cross-Stack Integration Rules'
---

# Sprintly Cross-Stack Integration Rules

- Treat every rule in this file as a strict requirement, not a suggestion.
- If backend DTOs/routes/response fields change, update frontend API usage in `public/` within the same task.
- If frontend behavior requires contract changes, update controllers/routes/DTOs in the same task.
- Keep response compatibility strict: frontend must consume `ApiResponse<T>` shape (`success`, `status_code`, `message`, `data`).
- Keep error compatibility strict: backend errors must continue to come from `ApiError` and remain parseable by frontend flows.
- Do not merge changes that leave backend and frontend out of sync.

# For Now We Only work with the backend/api. (DO NOT GENERATE FRONTEND CODE YET UNTIL I ASK YOU TO DO SO)

---
description: 'Use when implementing or modifying Sprintly backend code in Rust/Axum/SQLx, including controllers, routes, auth, DTO validation, utils, and migrations. Enforces strict backend architecture and error/response patterns.'
name: 'Sprintly Backend Strict Conventions'
applyTo: 'src/**/*.rs, migrations/**/*.sql, API.http'
---

# Sprintly Backend Strict Conventions

- Treat every rule in this file as a strict requirement, not a suggestion.
- Use Rust with async/await for all I/O, especially DB and HTTP handlers.
- Keep the architecture flow per resource as models -> controllers -> routes.
- Prefer UUID primary keys for users, tasks, projects, workspaces, and lookup entities.
- Use SQLx idioms (`query_as`, `FromRow`) and map DB errors to HTTP status codes (`404` not found, `500` internal error).
- In Axum handlers, use `Router::new().route(...)` and shared state through `State`.
- Use `chrono::Utc` or `chrono::NaiveDateTime` for timestamps.
- Keep naming consistent: `snake_case` for functions and `PascalCase` for structs.
- Keep imports clean and grouped as standard, external, then internal.
- Prefer readable multiline SQL for complex queries.
- For many-to-many task assignments, use join tables (for example `task_assignees`) with proper foreign keys.
- Keep migrations in `migrations/`, add constraints deliberately (`UNIQUE`, foreign keys with `ON DELETE CASCADE` where appropriate), and use `uuid-ossp` for UUID generation when needed.

## Controller Error/Response Pattern (Strict)

- Follow the controller style used in `src/controllers/auth_controller.rs` for new and modified handlers.
- Controller signatures must return `Result<Json<ApiResponse<T>>, ApiError>`.
- Success responses must use `ApiResponse::new(...)` from `src/utils/response.rs`.
- Failures must use `ApiError` variants from `src/utils/api_error.rs`; do not return ad-hoc JSON/string errors.
- For DTO validation, call `payload.validate()` and convert errors using `format_validation_errors(...)` from `src/utils/mod.rs`, then return `ApiError::BadRequest(...)`.
- Map database/crypto/JWT failures with `map_err(...)` into the correct `ApiError` variant.
- Keep structured tracing logs (`tracing::info!`, `tracing::warn!`, `tracing::error!`) for key controller actions and failures.
- Return API responses using `ApiResponse<T>` with consistent keys: `success`, `status_code`, `message`, `data`.

---
description: 'Use when implementing or modifying Sprintly static UI in public HTML/CSS/JS files. Enforces strict public-folder conventions and backend response compatibility.'
name: 'Sprintly Frontend Strict Conventions'
applyTo: 'public/**/*.html, public/**/*.css, public/**/*.js'
---

# Sprintly Frontend Strict Conventions

- Treat every rule in this file as a strict requirement, not a suggestion.
- Keep frontend implementation simple and framework-free (plain HTML/CSS/JS) unless explicitly requested otherwise.
- Preserve existing page structure and style conventions across `public/index.html`, `public/login.html`, `public/register.html`, `public/styles.css`, and `public/table.js`.
- Keep static pages compatible with backend routes and auth flow.
- When changing frontend API interactions, keep endpoint paths and HTTP methods aligned with backend routes.
- Parse and handle API responses using the established response shape: `success`, `status_code`, `message`, `data`.
- Do not introduce response contracts on the frontend that differ from `ApiResponse<T>` returned by backend controllers.
