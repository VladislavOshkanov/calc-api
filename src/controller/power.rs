use crate::AppState;
use crate::model::power::{CreatePower, Power};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

/// POST-эндпоинт для добавления нового объекта "Power".
pub async fn add_power(
    State(state): State<Arc<AppState>>,
    Json(create_power): Json<CreatePower>,
) -> Json<Power> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("powers");

    let power = Power {
        id: None,
        min_power: create_power.min_power,
        max_power: create_power.max_power,
        coefficient: create_power.coefficient,
    };

    let result = collection
        .insert_one(power.clone())
        .await
        .expect("Failed to insert power.");

    let mut power_with_id = power;
    power_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(power_with_id)
}

/// GET-эндпоинт для получения списка всех объектов "Power".
pub async fn get_powers(State(state): State<Arc<AppState>>) -> Json<Vec<Power>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Power>("powers");

    let mut cursor = collection
        .find(doc! {})
        .await
        .expect("Failed to fetch powers");

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

    match collection.find_one(filter).await {
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
            "coefficient": power.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
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

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Power with ID {id} deleted.")),
        Ok(_) => Err("Power not found.".to_string()),
        Err(err) => Err(format!("Failed to delete power: {err}")),
    }
}
