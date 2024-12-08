use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    middleware::Next,
};

/// Middleware для проверки авторизации.
pub async fn admin_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let admin_token = std::env::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");

    // Проверяем заголовок Authorization
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == format!("Bearer {}", admin_token) {
                return Ok(next.run(req).await);
            }
        }
    }


    // Если токен отсутствует или некорректный, возвращаем ошибку
    Err(StatusCode::UNAUTHORIZED)
}