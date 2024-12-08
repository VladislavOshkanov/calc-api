use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use std::sync::Arc;
use crate::{AppState, model::Power};

/// POST-эндпоинт для добавления нового объекта "Power".
pub async fn add_power(
    State(state): State<Arc<AppState>>,
    Json(power): Json<Power>,
) -> Json<Power> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("powers");

    collection
        .insert_one(power.clone(), None)
        .await
        .expect("Failed to insert power.");

    Json(power)
}

/// GET-эндпоинт для получения списка всех объектов "Power".
pub async fn get_powers(State(state): State<Arc<AppState>>) -> Json<Vec<Power>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Power>("powers");

    let mut cursor = collection.find(None, None).await.expect("Failed to fetch powers");

    let mut powers = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        powers.push(doc);
    }

    Json(powers)
}

/// GET-эндпоинт для получения объекта "Power" по ID.
pub async fn get_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Power>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Power>("powers");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(power)) => Ok(Json(power)),
        Ok(None) => Err("Power not found.".to_string()),
        Err(err) => Err(format!("Failed to fetch power: {err}")),
    }
}

/// PUT-эндпоинт для обновления объекта "Power" по ID.
pub async fn update_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(power): Json<Power>,
) -> Result<Json<Power>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Power>("powers");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "min_power": power.min_power,
            "max_power": power.max_power,
            "coefficient": power.coefficent,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(power)),
        Ok(_) => Err("Power not found.".to_string()),
        Err(err) => Err(format!("Failed to update power: {err}")),
    }
}

/// DELETE-эндпоинт для удаления объекта "Power" по ID.
pub async fn delete_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Power>("powers");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Power with ID {id} deleted.")),
        Ok(_) => Err("Power not found.".to_string()),
        Err(err) => Err(format!("Failed to delete power: {err}")),
    }
}