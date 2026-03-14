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
