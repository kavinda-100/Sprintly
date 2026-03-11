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
- 🔒 **Google OAuth + JWT Auth** – Secure authentication system
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

## 📁 Project Structure (coming soon)

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
