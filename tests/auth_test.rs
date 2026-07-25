use anyhow::Result;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
    middleware,
    routing::get,
};
use openapi::auth::auth_middleware::admin_auth;
use tower::ServiceExt;

const ADMIN_TOKEN: &str = "test_admin_token";

#[tokio::test]
async fn test_auth_middleware() -> Result<()> {
    let app = Router::new()
        .route("/admin/place", get(|| async { StatusCode::OK }))
        .layer(middleware::from_fn(admin_auth));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/admin/place")
                .body(Body::empty())
                .unwrap(),
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/admin/place")
                .header(header::AUTHORIZATION, "Bearer wrong_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/admin/place")
                .header(header::AUTHORIZATION, format!("Bearer {}", ADMIN_TOKEN))
                .body(Body::empty())
                .unwrap(),
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
