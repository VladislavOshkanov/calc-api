use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use mongodb::Client;
use openapi::{AppState, build_router_with_admin_token};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;

async fn app(uri: &str) -> Router {
    let client = Client::with_uri_str(uri).await.unwrap();
    build_router_with_admin_token(
        Arc::new(AppState { db_client: client }),
        "regression-secret".into(),
    )
}
async fn request(
    app: &Router,
    method: &str,
    path: &str,
    payload: Option<Value>,
) -> (StatusCode, Vec<u8>) {
    let req = Request::builder()
        .method(method)
        .uri(path)
        .header("Authorization", "Bearer regression-secret")
        .header("Content-Type", "application/json")
        .body(Body::from(
            payload.map(|v| v.to_string()).unwrap_or_default(),
        ))
        .unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    (
        status,
        to_bytes(response.into_body(), 1_000_000)
            .await
            .unwrap()
            .to_vec(),
    )
}
#[tokio::test]
async fn static_paths_cannot_escape_and_assets_still_work() {
    let app = app("mongodb://127.0.0.1:27017").await;
    for path in [
        "/static/%2e%2e/Cargo.toml",
        "/static/../Cargo.toml",
        "/static/%2Fetc%2Fpasswd",
        "/static/unknown.js",
    ] {
        assert_eq!(
            request(&app, "GET", path, None).await.0,
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }
    assert_eq!(
        request(&app, "GET", "/static/calculator.js", None).await.0,
        StatusCode::OK
    );
}
#[tokio::test]
async fn invalid_ids_and_payloads_have_error_statuses_and_json() {
    let app = app("mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=100").await;
    for method in ["GET", "DELETE"] {
        let (status, body) = request(&app, method, "/admin/place/invalid", None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(serde_json::from_slice::<Value>(&body).unwrap()["message"].is_string());
    }
    for (kind, payload) in [
        ("place", json!({"name":"", "coefficient":1})),
        ("place", json!({"name":"test", "coefficient":0})),
        (
            "power",
            json!({"min_power":100,"max_power":50,"coefficient":1}),
        ),
        ("kbm", json!({"class":14,"coefficient":1})),
        (
            "age_experience",
            json!({"age":20,"experience":10,"coefficient":1}),
        ),
        ("season", json!({"months":13,"coefficient":1})),
        ("limitation", json!({"limited":true,"coefficient":-1})),
        (
            "base_price",
            json!({"min_base_price":200,"max_base_price":100}),
        ),
    ] {
        assert_eq!(
            request(
                &app,
                "POST",
                &format!("/admin/{kind}"),
                Some(payload.clone())
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
        let mut update = payload;
        if kind == "base_price" {
            update["created_at"] = json!({"$date":{"$numberLong":"0"}});
        }
        assert_eq!(
            request(
                &app,
                "PUT",
                &format!("/admin/{kind}/000000000000000000000001"),
                Some(update)
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
    let (status, body) = request(&app, "POST", "/admin/place", Some(json!({}))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(serde_json::from_slice::<Value>(&body).unwrap()["message"].is_string());
}
#[tokio::test]
async fn database_failure_returns_503_and_router_keeps_serving() {
    let app = app("mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=100").await;
    for path in ["/api/all-models", "/admin/place"] {
        let (status, body) = request(&app, "GET", path, None).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap()["message"],
            "Database unavailable"
        );
    }
    assert_eq!(
        request(&app, "GET", "/static/admin.js", None).await.0,
        StatusCode::OK
    );
}
#[tokio::test]
async fn known_default_token_is_rejected() {
    let app = app("mongodb://127.0.0.1:27017").await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/admin/place")
                .header("Authorization", "Bearer test_admin_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "requires MongoDB on port 27017"]
async fn mongo_calculation_latest_tariff_and_missing_records() {
    let db_name = format!("openapi_regression_{}", uuid::Uuid::new_v4().simple());
    let uri = format!("mongodb://127.0.0.1:27017/{db_name}?serverSelectionTimeoutMS=2000");
    let client = Client::with_uri_str(&uri).await.unwrap();
    let app = build_router_with_admin_token(
        Arc::new(AppState {
            db_client: client.clone(),
        }),
        "regression-secret".into(),
    );
    let mut ids = serde_json::Map::new();
    for (kind, payload) in [
        (
            "place",
            json!({"name":"Regression place","coefficient":1.5}),
        ),
        (
            "power",
            json!({"min_power":100,"max_power":150,"coefficient":1.2}),
        ),
        ("kbm", json!({"class":3,"coefficient":0.8})),
        (
            "age_experience",
            json!({"age":30,"experience":10,"coefficient":1.1}),
        ),
        ("season", json!({"months":9,"coefficient":0.95})),
        ("limitation", json!({"limited":true,"coefficient":1})),
    ] {
        let (status, body) = request(&app, "POST", &format!("/admin/{kind}"), Some(payload)).await;
        assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
        let item: Value = serde_json::from_slice(&body).unwrap();
        ids.insert(format!("{kind}_id"), item["_id"]["$oid"].clone());
    }
    // Explicit timestamps make the latest-tariff assertion deterministic.
    let base = client
        .database(&db_name)
        .collection::<mongodb::bson::Document>("base_prices");
    base.insert_many([
        mongodb::bson::doc!{"min_base_price":100.0,"max_base_price":200.0,"created_at":mongodb::bson::DateTime::from_millis(2000)},
        mongodb::bson::doc!{"min_base_price":1.0,"max_base_price":2.0,"created_at":mongodb::bson::DateTime::from_millis(1000)},
    ]).await.unwrap();
    let (status, body) = request(
        &app,
        "POST",
        "/api/calculate-coefficient",
        Some(Value::Object(ids.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
    let result: Value = serde_json::from_slice(&body).unwrap();
    let expected = 1.5 * 1.2 * 0.8 * 1.1;
    assert!((result["min_price"].as_f64().unwrap() - expected * 100.0).abs() < 1e-9);
    assert!((result["max_price"].as_f64().unwrap() - expected * 200.0).abs() < 1e-9);
    assert_eq!(result["coefficients"]["season"], 1.0);
    let (_, body) = request(
        &app,
        "POST",
        "/admin/limitation",
        Some(json!({"limited":false,"coefficient":3.16})),
    )
    .await;
    ids.insert(
        "limitation_id".into(),
        serde_json::from_slice::<Value>(&body).unwrap()["_id"]["$oid"].clone(),
    );
    let (status, body) = request(
        &app,
        "POST",
        "/api/calculate-coefficient",
        Some(Value::Object(ids.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let result: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(result["coefficients"]["kbm"], 1.0);
    assert_eq!(result["coefficients"]["age_experience"], 1.0);
    assert!((result["min_price"].as_f64().unwrap() - 1.5 * 1.2 * 3.16 * 100.0).abs() < 1e-9);
    for method in ["GET", "PUT", "DELETE"] {
        let payload = (method == "PUT").then(|| json!({"name":"missing","coefficient":1}));
        assert_eq!(
            request(
                &app,
                method,
                "/admin/place/000000000000000000000001",
                payload
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
    }
    ids.insert("place_id".into(), json!("000000000000000000000001"));
    assert_eq!(
        request(
            &app,
            "POST",
            "/api/calculate-coefficient",
            Some(Value::Object(ids))
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // Drop only the unique database created by this test.
    client.database(&db_name).drop().await.unwrap();
}

#[test]
fn startup_rejects_missing_or_empty_admin_token() {
    for token in [None, Some("   ")] {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_openapi"));
        command
            .env_remove("ADMIN_TOKEN")
            .env("MONGODB_URI", "mongodb://127.0.0.1:27017");
        if let Some(token) = token {
            command.env("ADMIN_TOKEN", token);
        }
        let output = command.output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("ADMIN_TOKEN"));
    }
}

#[test]
fn legacy_coefficent_is_read_but_serialization_uses_coefficient() {
    use mongodb::bson::{doc, from_document, to_document};
    use openapi::model::{place::Place, power::Power};
    let place: Place = from_document(doc! {"name":"Legacy place", "coefficent":1.5}).unwrap();
    let power: Power =
        from_document(doc! {"min_power":100, "max_power":150, "coefficent":1.2}).unwrap();
    assert_eq!(place.coefficient, 1.5);
    assert_eq!(power.coefficient, 1.2);
    for value in [to_document(&place).unwrap(), to_document(&power).unwrap()] {
        assert!(value.contains_key("coefficient"));
        assert!(!value.contains_key("coefficent"));
    }
}
