use crate::model::place::{CreatePlace, Place};
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

/// POST-эндпоинт для добавления нового объекта "Place".
pub async fn add_place(
    State(state): State<Arc<AppState>>,
    Json(create_place): Json<CreatePlace>,
) -> Json<Place> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("places");

    let place = Place {
        id: None,
        name: create_place.name,
        coefficent: create_place.coefficent,
    };

    let result = collection
        .insert_one(place.clone())
        .await
        .expect("Failed to insert place.");

    let mut place_with_id = place;
    place_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(place_with_id)
}

/// GET-эндпоинт для получения списка всех объектов "Place".
pub async fn get_places(State(state): State<Arc<AppState>>) -> Json<Vec<Place>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Place>("places");

    let mut cursor = collection
        .find(doc! {})
        .await
        .expect("Failed to fetch places");

    let mut places = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        places.push(doc);
    }

    Json(places)
}

/// GET-эндпоинт для получения объекта Place по ID.
pub async fn get_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Place>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(place)) => Ok(Json(place)),
        Ok(None) => Err("Place not found".to_string()),
        Err(err) => Err(format!("Failed to fetch place: {err}")),
    }
}

/// PUT-эндпоинт для обновления объекта "Place" по ID.
pub async fn update_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(place): Json<Place>,
) -> Result<Json<Place>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "name": place.name.clone(),
            "coefficient": place.coefficent,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(place)),
        Ok(_) => Err("Place not found.".to_string()),
        Err(err) => Err(format!("Failed to update place: {err}")),
    }
}

/// DELETE-эндпоинт для удаления объекта "Place" по ID.
pub async fn delete_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Place with ID {id} deleted.")),
        Ok(_) => Err("Place not found.".to_string()),
        Err(err) => Err(format!("Failed to delete place: {err}")),
    }
}
