use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::place::{CreatePlace, Place};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

/// POST-эндпоинт для добавления нового объекта "Place".
pub async fn add_place(
    State(state): State<Arc<AppState>>,
    ApiJson(create_place): ApiJson<CreatePlace>,
) -> Result<Json<Place>, ApiError> {
    create_place.validate()?;
    let collection = state.database().collection("places");

    let place = Place {
        id: None,
        name: create_place.name,
        coefficient: create_place.coefficient,
    };

    let result = collection.insert_one(place.clone()).await?;

    let mut place_with_id = place;
    place_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(place_with_id))
}

/// GET-эндпоинт для получения списка всех объектов "Place".
pub async fn get_places(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Place>>, ApiError> {
    let collection = state.database().collection::<Place>("places");

    let mut cursor = collection.find(doc! {}).await?;

    let mut places = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        places.push(doc);
    }

    Ok(Json(places))
}

/// GET-эндпоинт для получения объекта Place по ID.
pub async fn get_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Place>, ApiError> {
    let collection = state.database().collection::<Place>("places");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(place)) => Ok(Json(place)),
        Ok(None) => Err(ApiError::not_found("Place not found")),
        Err(err) => Err(err.into()),
    }
}

/// PUT-эндпоинт для обновления объекта "Place" по ID.
pub async fn update_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(place): ApiJson<Place>,
) -> Result<Json<Place>, ApiError> {
    place.validate()?;
    let collection = state.database().collection::<Place>("places");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$unset": { "coefficent": "" },
        "$set": {
            "name": place.name.clone(),
            "coefficient": place.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(place)),
        Ok(_) => Err(ApiError::not_found("Place not found.")),
        Err(err) => Err(err.into()),
    }
}

/// DELETE-эндпоинт для удаления объекта "Place" по ID.
pub async fn delete_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<Place>("places");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Place with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("Place not found.")),
        Err(err) => Err(err.into()),
    }
}
