pub mod auth;
pub mod controller;
pub mod error;
pub mod model;

use crate::controller::age_experience::*;
use crate::controller::all_models::*;
use crate::controller::base_price::*;
use crate::controller::coefficient_calculator::*;
use crate::controller::kbm::*;
use crate::controller::limitation::*;
use crate::controller::place::*;
use crate::controller::power::*;
use crate::controller::season::*;
use axum::extract::Path;
use axum::response::Html;
use axum::{
    Router,
    http::StatusCode,
    middleware,
    routing::{get, post},
};
use mongodb::Client;
use std::sync::Arc;
use tokio::fs;

use crate::auth::auth_middleware::admin_auth;

pub struct AppState {
    pub db_client: Client,
}

impl AppState {
    pub fn database(&self) -> mongodb::Database {
        self.db_client
            .default_database()
            .unwrap_or_else(|| self.db_client.database("openapi"))
    }
}

pub fn build_router(shared_state: Arc<AppState>) -> Router {
    let token = std::env::var("ADMIN_TOKEN").expect("ADMIN_TOKEN must be set");
    assert!(!token.trim().is_empty(), "ADMIN_TOKEN must not be empty");
    build_router_with_admin_token(shared_state, token)
}

pub fn build_router_with_admin_token(shared_state: Arc<AppState>, token: String) -> Router {
    assert!(!token.trim().is_empty(), "ADMIN_TOKEN must not be empty");
    let admin_routes = Router::new()
        .route("/admin/place", post(add_place).get(get_places))
        .route(
            "/admin/place/{id}",
            get(get_place).put(update_place).delete(delete_place),
        )
        .route("/admin/power", post(add_power).get(get_powers))
        .route(
            "/admin/power/{id}",
            get(get_power).put(update_power).delete(delete_power),
        )
        .route("/admin/kbm", post(add_kbm).get(get_kbms))
        .route(
            "/admin/kbm/{id}",
            get(get_kbm).put(update_kbm).delete(delete_kbm),
        )
        .route(
            "/admin/age_experience",
            post(add_age_experience).get(get_age_experiences),
        )
        .route(
            "/admin/age_experience/{id}",
            get(get_age_experience)
                .put(update_age_experience)
                .delete(delete_age_experience),
        )
        .route("/admin/season", post(add_season).get(get_seasons))
        .route(
            "/admin/season/{id}",
            get(get_season).put(update_season).delete(delete_season),
        )
        .route(
            "/admin/limitation",
            post(add_limitation).get(get_limitations),
        )
        .route(
            "/admin/limitation/{id}",
            get(get_limitation)
                .put(update_limitation)
                .delete(delete_limitation),
        )
        .route(
            "/admin/base_price",
            post(add_base_price).get(get_base_prices),
        )
        .route(
            "/admin/base_price/{id}",
            get(get_base_price)
                .put(update_base_price)
                .delete(delete_base_price),
        )
        .layer(middleware::from_fn_with_state(token, admin_auth));

    Router::new()
        .route("/api/all-models", get(get_all_models))
        .route("/api/calculate-coefficient", post(calculate_coefficient))
        .route("/", get(root))
        .route("/admin", get(admin_root))
        .route("/admin/", get(admin_root))
        .route("/static/{*file}", get(static_files))
        .merge(admin_routes)
        .with_state(shared_state)
}

async fn root() -> Result<Html<String>, (StatusCode, String)> {
    render_html("./static/index.html").await
}

async fn admin_root() -> Result<Html<String>, (StatusCode, String)> {
    render_html("./static/admin.html").await
}

async fn render_html(path: &str) -> Result<Html<String>, (StatusCode, String)> {
    let content = fs::read_to_string(path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("read error: {}", e),
        )
    })?;
    Ok(Html(content))
}

async fn static_files(
    Path(file): Path<String>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), (StatusCode, String)> {
    // Only explicit bundled assets are exposed; filesystem paths and symlinks are never read.
    let data: &[u8] = match file.as_str() {
        "index.html" => include_bytes!("../static/index.html"),
        "admin.html" => include_bytes!("../static/admin.html"),
        "style.css" => include_bytes!("../static/style.css"),
        "admin.js" => include_bytes!("../static/admin.js"),
        "calculator.js" => include_bytes!("../static/calculator.js"),
        _ => return Err((StatusCode::NOT_FOUND, "Not found".into())),
    };
    let mut headers = axum::http::HeaderMap::new();
    let mime = mime_guess::from_path(&file).first_or_octet_stream();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok((headers, data.to_vec()))
}
