use mongodb::Client;
use openapi::{build_router, AppState};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to initialize MongoDB client.");

    let shared_state = Arc::new(AppState { db_client });
    let router = build_router(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on {}", addr);
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
