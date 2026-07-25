# CRUSH.md — Project Agent Guide

## Build, Lint, and Test Commands

- **Build**:  
  `cargo build`

- **Run the app**:  
  `cargo run --bin openapi`  
  (Server listens on port 8000 by default)

- **Seed reference data**:  
  `cargo run --bin seed_reference_data`

- **Seed places**:  
  `cargo run --bin seed_places`

- **Test all**:  
  `cargo test`

- **Test a single test**:  
  `cargo test test_name`  
  (Example: `cargo test test_auth_middleware`)

- **List tests**:  
  `cargo test -- --list`

- **Lint (check with clippy)**:  
  `cargo clippy --all-targets --all-features -- -D warnings`

- **Format check**:  
  `cargo fmt -- --check`

---

## Code Style Guidelines

- **Imports**:  
  - Group imports: std, 3rd-party, local (crate::)  
  - Prefer single-line with braces for grouped items, multi-line for many imports

- **Formatting**:  
  - Enforce with `cargo fmt`
  - Indent 4 spaces, trim trailing whitespace
  - Keep function signatures and struct definitions compact, wrap for 80–100 chars

- **Types**:  
  - Use explicit types for all struct fields and function parameters/results
  - Use Option<T> for optional fields, Result<T, E> for fallible operations

- **Naming conventions**:  
  - snake_case for vars, fields, and functions  
  - CamelCase for structs, enums, traits  
  - UPPER_SNAKE_CASE for constants  
  - Modules: snake_case, files/folders match module names

- **Error Handling**:  
  - Use `Result<T, E>` for public API, with string error or anyhow
  - For DB and external failures, use `.map_err(|e| format!(...))?` or `.expect("msg")` for unrecoverable init/config
  - DO NOT panic in production logic; propagate errors

- **Unwrap/expect**:  
  - Use `.expect` only in startup/config/test code  
  - Never use `.unwrap` in handlers/endpoints; use pattern matching or error propagation

- **Documentation & Comments**:  
  - Use rustdoc `///` for all public functions and structs  
  - Write doc comments in Russian where suitable, otherwise English (see existing style)

- **Testing**:  
  - Integration and auth test files live in `/tests`  
  - Use `#[tokio::test]` for async tests  
  - Prefer `anyhow::Result<()>` as test return for convenient `?` usage

- **Other**:  
  - Environment variables (see `ADMIN_TOKEN` in auth_middleware)
  - No comments in code unless required for rustdoc or documentation clarification
