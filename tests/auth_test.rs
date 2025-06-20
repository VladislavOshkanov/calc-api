use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

const BASE_URL: &str = "http://localhost:8000";
const ADMIN_TOKEN: &str = "test_admin_token";

#[tokio::test]
async fn test_auth_middleware() -> Result<()> {
    // Тест 1: Запрос без токена должен вернуть 401
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = client
        .get(&format!("{}/admin/place", BASE_URL))
        .send()
        .await?;

    println!("Response status without token: {}", response.status());
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

    // Тест 2: Запрос с неправильным токеном должен вернуть 401
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", "Bearer wrong_token".parse().unwrap());

    let client_with_wrong_token = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = client_with_wrong_token
        .get(&format!("{}/admin/place", BASE_URL))
        .send()
        .await?;

    println!("Response status with wrong token: {}", response.status());
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

    // Тест 3: Запрос с правильным токеном должен пройти
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", ADMIN_TOKEN).parse().unwrap());

    let client_with_correct_token = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let response = client_with_correct_token
        .get(&format!("{}/admin/place", BASE_URL))
        .send()
        .await?;

    println!("Response status with correct token: {}", response.status());
    // Должен вернуть 200 или 404 (если нет данных), но не 401
    assert_ne!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

    Ok(())
} 