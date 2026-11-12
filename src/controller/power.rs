use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
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
    ApiJson(create_power): ApiJson<CreatePower>,
) -> Result<Json<Power>, ApiError> {
    create_power.validate()?;
    let collection = state.database().collection("powers");

    let power = Power {
        id: None,
        min_power: create_power.min_power,
        max_power: create_power.max_power,
        coefficient: create_power.coefficient,
    };

    let result = collection.insert_one(power.clone()).await?;

    let mut power_with_id = power;
    power_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(power_with_id))
}

/// GET-эндпоинт для получения списка всех объектов "Power".
pub async fn get_powers(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Power>>, ApiError> {
    let collection = state.database().collection::<Power>("powers");

    let mut cursor = collection.find(doc! {}).await?;

    let mut powers = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        powers.push(doc);
    }

    Ok(Json(powers))
}

/// GET-эндпоинт для получения объекта "Power" по ID.
pub async fn get_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Power>, ApiError> {
    let collection = state.database().collection::<Power>("powers");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(power)) => Ok(Json(power)),
        Ok(None) => Err(ApiError::not_found("Power not found.")),
        Err(err) => Err(err.into()),
    }
}

/// PUT-эндпоинт для обновления объекта "Power" по ID.
pub async fn update_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(power): ApiJson<Power>,
) -> Result<Json<Power>, ApiError> {
    power.validate()?;
    let collection = state.database().collection::<Power>("powers");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$unset": { "coefficent": "" },
        "$set": {
            "min_power": power.min_power,
            "max_power": power.max_power,
            "coefficient": power.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(power)),
        Ok(_) => Err(ApiError::not_found("Power not found.")),
        Err(err) => Err(err.into()),
    }
}

/// DELETE-эндпоинт для удаления объекта "Power" по ID.
pub async fn delete_power(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<Power>("powers");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Power with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("Power not found.")),
        Err(err) => Err(err.into()),
    }
}
