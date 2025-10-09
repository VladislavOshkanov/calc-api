use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

/// Middleware для проверки авторизации.
pub async fn admin_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Получаем токен из переменной окружения, если не установлен - используем дефолтный
    let admin_token =
        std::env::var("ADMIN_TOKEN").unwrap_or_else(|_| "test_admin_token".to_string());

    // Проверяем заголовок Authorization
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", admin_token) {
                return Ok(next.run(req).await);
            } else {
                println!("❌ Auth middleware: Token mismatch");
            }
        } else {
            println!("❌ Auth middleware: Invalid header encoding");
        }
    } else {
        println!("❌ Auth middleware: No Authorization header");
    }

    // Если токен отсутствует или некорректный, возвращаем ошибку
    println!("🚫 Auth middleware: Access denied");
    Err(StatusCode::UNAUTHORIZED)
}
