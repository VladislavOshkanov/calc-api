use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use std::sync::Arc;
use crate::{AppState, model::Kbm};

/// POST-эндпоинт для добавления нового объекта "Kbm".
pub async fn add_kbm(
    State(state): State<Arc<AppState>>,
    Json(kbm): Json<Kbm>,
) -> Json<Kbm> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("kbms");

    collection
        .insert_one(kbm.clone(), None)
        .await
        .expect("Failed to insert kbm.");

    Json(kbm)
}

/// GET-эндпоинт для получения списка всех объектов "Kbm".
pub async fn get_kbms(State(state): State<Arc<AppState>>) -> Json<Vec<Kbm>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Kbm>("kbms");

    let mut cursor = collection.find(None, None).await.expect("Failed to fetch kbms");

    let mut kbms = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        kbms.push(doc);
    }

    Json(kbms)
}

/// GET-эндпоинт для получения объекта "Kbm" по ID.
pub async fn get_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Kbm>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(kbm)) => Ok(Json(kbm)),
        Ok(None) => Err("Kbm not found.".to_string()),
        Err(err) => Err(format!("Failed to fetch kbm: {err}")),
    }
}

/// PUT-эндпоинт для обновления объекта "Kbm" по ID.
pub async fn update_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(kbm): Json<Kbm>,
) -> Result<Json<Kbm>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "coefficient": kbm.coefficient,
            "class": kbm.class,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(kbm)),
        Ok(_) => Err("Kbm not found.".to_string()),
        Err(err) => Err(format!("Failed to update kbm: {err}")),
    }
}

/// DELETE-эндпоинт для удаления объекта "Kbm" по ID.
pub async fn delete_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Kbm with ID {id} deleted.")),
        Ok(_) => Err("Kbm not found.".to_string()),
        Err(err) => Err(format!("Failed to delete kbm: {err}")),
    }
}