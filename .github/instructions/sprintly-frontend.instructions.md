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
