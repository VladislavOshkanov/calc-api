use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::base_price::{BasePrice, CreateBasePrice};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{DateTime, doc, oid::ObjectId};
use std::sync::Arc;

/// POST-эндпоинт для добавления нового объекта "BasePrice".
pub async fn add_base_price(
    State(state): State<Arc<AppState>>,
    ApiJson(create_base_price): ApiJson<CreateBasePrice>,
) -> Result<Json<BasePrice>, ApiError> {
    create_base_price.validate()?;
    let collection = state.database().collection("base_prices");

    let base_price = BasePrice {
        id: None,
        min_base_price: create_base_price.min_base_price,
        max_base_price: create_base_price.max_base_price,
        created_at: DateTime::now(),
    };

    let result = collection.insert_one(base_price.clone()).await?;

    let mut base_price_with_id = base_price;
    base_price_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(base_price_with_id))
}

/// GET-эндпоинт для получения списка всех объектов "BasePrice".
pub async fn get_base_prices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BasePrice>>, ApiError> {
    let collection = state.database().collection::<BasePrice>("base_prices");

    let mut cursor = collection.find(doc! {}).await?;

    let mut base_prices = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        base_prices.push(doc);
    }

    Ok(Json(base_prices))
}

/// GET-эндпоинт для получения объекта "BasePrice" по ID.
pub async fn get_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<BasePrice>, ApiError> {
    let collection = state.database().collection::<BasePrice>("base_prices");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(base_price)) => Ok(Json(base_price)),
        Ok(None) => Err(ApiError::not_found("BasePrice not found.")),
        Err(err) => Err(err.into()),
    }
}

/// PUT-эндпоинт для обновления объекта "BasePrice" по ID.
pub async fn update_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(base_price): ApiJson<BasePrice>,
) -> Result<Json<BasePrice>, ApiError> {
    base_price.validate()?;
    let collection = state.database().collection::<BasePrice>("base_prices");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "min_base_price": base_price.min_base_price,
            "max_base_price": base_price.max_base_price,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(base_price)),
        Ok(_) => Err(ApiError::not_found("BasePrice not found.")),
        Err(err) => Err(err.into()),
    }
}

/// DELETE-эндпоинт для удаления объекта "BasePrice" по ID.
pub async fn delete_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<BasePrice>("base_prices");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("BasePrice with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("BasePrice not found.")),
        Err(err) => Err(err.into()),
    }
}
