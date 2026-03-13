# 🏃 Sprintly API

A production-ready Task Management REST API built with **Rust**, **Axum**, and **SQLx**, designed for team collaboration and task tracking. This project demonstrates clean architecture, async Rust patterns, and scalable backend design.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)  
[![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)](https://github.com/tokio-rs/axum)  
[![SQLx](https://img.shields.io/badge/sqlx-0.7-green.svg)](https://github.com/launchbadge/sqlx)  
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## ✨ Features

- 🏗️ **Clean Architecture** – Organized into Models, Controllers, Services, Routes, and Middleware
- 🔄 **API Versioning** – `/api/v1` ready
- 🔐 **JWT Cookie Authentication (Implemented)** – Register, login, logout, and protected `me` endpoint
- 🔒 **Google OAuth (Planned/Partial)** – User model and login checks are ready for OAuth users
- 🗃️ **Database Migrations** – SQLx-powered migrations for PostgreSQL
- ⚡ **Async/Await** – Powered by Tokio runtime for high-performance APIs
- 📝 **Full CRUD** – Tasks, Projects, Users, Workspaces
- 🩺 **Health Check** – `/health` endpoint for monitoring
- 📦 **Consistent Responses** – Standardized JSON format across all endpoints

---

## 🛠️ Technologies Used

| Technology             | Purpose                                         |
| ---------------------- | ----------------------------------------------- |
| **Rust**               | Programming language                            |
| **Axum**               | Web framework                                   |
| **SQLx**               | Async SQL toolkit with compile-time query check |
| **Tokio**              | Async runtime                                   |
| **PostgreSQL**         | Relational database                             |
| **Serde**              | Serialization/deserialization                   |
| **Chrono**             | Date and time handling                          |
| **dotenvy**            | Environment variable management                 |
| **JWT**                | Token-based authentication                      |
| **OAuth2**             | Google authentication                           |
| **Tower / Tower-HTTP** | Middleware and utilities                        |

---

## 📁 Project Structure (Current)

```bash
src/
├── controllers/
│   ├── auth_controller.rs      # register/login/logout/me handlers
│   └── root_controller.rs      # health check
├── dtos/
│   └── auth_dto.rs             # request validation + auth response DTO
├── middleware/
│   └── auth.rs                 # JWT cookie auth extractor for protected routes
├── routes/
│   ├── auth_routes.rs          # /api/v1/auth/* route definitions
│   └── mod.rs                  # route composition + static file fallback
├── utils/
│   ├── response.rs             # ApiResponse<T> success wrapper
│   ├── api_error.rs            # ApiError enum -> HTTP error responses
│   ├── jwt.rs                  # token generation and verification
│   ├── hash.rs                 # Argon2 password hash + verify
│   └── mod.rs                  # utility exports + validation error formatter
├── db/
│   └── mod.rs                  # DB pool + automatic SQLx migrations at startup
└── main.rs                     # app bootstrap, middleware layers, state wiring
```

## 🔐 Auth Module (Implemented So Far)

Base path: `/api/v1/auth`

- `POST /register`
    - Validates payload (`name`, `email`, `password`, `confirm_password`)
    - Checks duplicate email
    - Hashes password with Argon2
    - Creates user in PostgreSQL
    - Generates JWT and stores it in an HTTP-only cookie named `token`
    - Returns normalized `ApiResponse<AuthUserResponse>`

- `POST /login`
    - Validates payload
    - Finds user by email
    - Blocks password login for Google-only users (`google_id` present)
    - Verifies password against Argon2 hash
    - Generates JWT and updates `token` cookie
    - Returns normalized `ApiResponse<AuthUserResponse>`

- `POST /logout`
    - Clears auth cookie (`token`)
    - Returns normalized `ApiResponse<()>`

- `GET /me`
    - Protected route using `AuthUser` extractor (`src/middleware/auth.rs`)
    - Reads and verifies JWT from cookie
    - Loads user from DB
    - Returns current authenticated user profile in `ApiResponse<AuthUserResponse>`

## 🧰 `src/utils` Folder and Use Cases

This project is educational, so the utility layer is intentionally explicit and reusable for future modules (tasks, projects, workspaces).

| File                     | What it does                                                                                                                         | Why it matters for future features                            |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------- |
| `src/utils/response.rs`  | Defines `ApiResponse<T>` with `success`, `status_code`, `message`, `data`                                                            | Gives every endpoint a predictable success payload shape      |
| `src/utils/api_error.rs` | Defines `ApiError` (`BadRequest`, `Unauthorized`, `NotFound`, `Conflict`, `InternalServerError`) and converts it into HTTP responses | Centralizes error-to-status mapping so controllers stay clean |
| `src/utils/jwt.rs`       | Generates and verifies JWT claims (`sub`, `exp`)                                                                                     | Reusable token logic for any protected resource               |
| `src/utils/hash.rs`      | Hashes passwords (Argon2 + random salt) and verifies passwords                                                                       | Security best practice reusable for account/password features |
| `src/utils/mod.rs`       | Exports utility modules and formats validator errors into readable strings                                                           | Keeps validation messages user-friendly and consistent        |

## 🔄 How Response + Error + JWT Work Together

1. **Request enters route** (`src/routes/auth_routes.rs`) and is handled by `auth_controller`.
2. **DTO validation runs** (`validator` crate in `src/dtos/auth_dto.rs`).
3. **If validation fails**, controller maps errors using `format_validation_errors(...)` and returns `ApiError::BadRequest(...)`.
4. **If business/database logic fails**, controller returns another `ApiError` variant.
5. **`ApiError` implements `IntoResponse`**, so Axum converts it into a standardized HTTP error payload.
6. **If success**, controller returns `Json(ApiResponse<T>)` from `src/utils/response.rs`.
7. **For protected routes**, `AuthUser` middleware extractor in `src/middleware/auth.rs` reads `token` from cookies, verifies JWT using `utils/jwt.rs`, fetches the user from DB, and injects authenticated `User` into the handler.

## 📦 Response Format (Current)

Success shape:

```json
{
	"success": true,
	"status_code": 200,
	"message": "User logged in successfully",
	"data": {
		"id": "uuid",
		"email": "user@example.com",
		"name": "User"
	}
}
```

Error shape:

```json
{
	"success": false,
	"status_code": 400,
	"message": "email: Invalid email format, password: Password must be between 6 and 12 characters",
	"data": null
}
```

## 🍪 JWT Cookie Behavior

- Cookie name: `token`
- `HttpOnly`: enabled (protects token from client-side JS access)
- `SameSite`: `Lax`
- `Secure`: enabled when `DEV_MODE != development`
- Signing secret: `JWT_SECRET` from environment

This makes auth state server-trusted and easy to consume from browser clients.

## database migrations

1. install sqlx-cli

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

2. create a migration

```bash
sqlx migrate add init_schema
```

3. run migrations

```bash
sqlx migrate run
# this step run is the manual way
# `src/db/mod.rs` has the code to run migrations automatically when the app starts
```

## Relationship Diagram

```ascii
+------------+        +-------------+        +----------+
|   users    |        | workspaces  |        | projects |
+------------+        +-------------+        +----------+
| id         |<-------| owner_id    |        | id       |
| email      |        | name        |<-------| workspace_id
| name       |        | created_at  |        | name
| google_id  |        +-------------+        | description
+------------+                               +----------+
       |                                          |
       |                                          |
       |                                          |
       v                                          v
+-------------+    +----------------+    +----------------+
|   tasks     |<---| task_assignees |--->|     users      |
+-------------+    +----------------+    +----------------+
| id          |    | task_id        |    | id             |
| project_id  |    | user_id        |    +----------------+
| title       |    +----------------+
| description |
| status_id   |----> task_status
| priority_id |----> task_priority
| owner_id    |
+-------------+
       |
       v
+-------------+
|  comments   |
+-------------+
| id          |
| task_id     |
| user_id     |
| content     |
+-------------+
```
