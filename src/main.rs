mod controller;
mod model;
mod auth;

use axum::{
    routing::{get, post}, Router, middleware
};
use mongodb::Client;
use std::sync::Arc;
use std::net::SocketAddr;
use crate::controller::place::*;
use crate::controller::power::*;
use crate::controller::kbm::*;
use crate::controller::age_experience::*;
use crate::controller::season::*;
use crate::controller::limitation::*;

use crate::auth::auth_middleware::admin_auth;


struct AppState {
    db_client: Client
}

#[tokio::main]
async fn main() {
    // Подключение к базе данных MongoDB
    let db_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to initialize MongoDB client.");

    // Настройка состояния приложения
    let shared_state = Arc::new(AppState { db_client });

    let admin_routes = Router::new()
        // Place маршруты
        .route("/admin/place", post(add_place).get(get_places))
        .route("/admin/place/:id", get(get_place).put(update_place).delete(delete_place))
        // Power маршруты
        .route("/admin/power", post(add_power).get(get_powers))
        .route("/admin/power/:id", get(get_power).put(update_power).delete(delete_power))
        // Kbm маршруты
        .route("/admin/kbm", post(add_kbm).get(get_kbms))
        .route("/admin/kbm/:id", get(get_kbm).put(update_kbm).delete(delete_kbm))
        // AgeExperience маршруты
        .route("/admin/age_experience", post(add_age_experience).get(get_age_experiences))
        .route("/admin/age_experience/:id", get(get_age_experience).put(update_age_experience).delete(delete_age_experience))
        // Season маршруты
        .route("/admin/season", post(add_season).get(get_seasons))
        .route("/admin/season/:id", get(get_season).put(update_season).delete(delete_season))
        // Limitation маршруты
        .route("/admin/limitation", post(add_limitation).get(get_limitations))
        .route("/admin/limitation/:id", get(get_limitation).put(update_limitation).delete(delete_limitation))
        // Применяем middleware к каждому запросу
        .layer(middleware::from_fn(admin_auth));

    // Настройка маршрутов
    let router = Router::new()
        .merge(admin_routes)
        .with_state(shared_state);


    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on {}", addr);
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}