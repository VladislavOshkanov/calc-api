use axum::{
    extract::{Path, State},
    Json,
};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use futures::TryStreamExt;
use std::sync::Arc;
use crate::AppState;
use crate::model::base_price::{BasePrice, CreateBasePrice};

/// POST-эндпоинт для добавления нового объекта "BasePrice".
pub async fn add_base_price(
    State(state): State<Arc<AppState>>,
    Json(create_base_price): Json<CreateBasePrice>,
) -> Json<BasePrice> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("base_prices");

    let base_price = BasePrice {
        id: None,
        min_base_price: create_base_price.min_base_price,
        max_base_price: create_base_price.max_base_price,
        created_at: DateTime::now(),
    };

    let result = collection
        .insert_one(base_price.clone(), None)
        .await
        .expect("Failed to insert base_price.");

    let mut base_price_with_id = base_price;
    base_price_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(base_price_with_id)
}

/// GET-эндпоинт для получения списка всех объектов "BasePrice".
pub async fn get_base_prices(State(state): State<Arc<AppState>>) -> Json<Vec<BasePrice>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<BasePrice>("base_prices");

    let mut cursor = collection.find(None, None).await.expect("Failed to fetch base_prices");

    let mut base_prices = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        base_prices.push(doc);
    }

    Json(base_prices)
}

/// GET-эндпоинт для получения объекта "BasePrice" по ID.
pub async fn get_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<BasePrice>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<BasePrice>("base_prices");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(base_price)) => Ok(Json(base_price)),
        Ok(None) => Err("BasePrice not found.".to_string()),
        Err(err) => Err(format!("Failed to fetch base_price: {err}")),
    }
}

/// PUT-эндпоинт для обновления объекта "BasePrice" по ID.
pub async fn update_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(base_price): Json<BasePrice>,
) -> Result<Json<BasePrice>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<BasePrice>("base_prices");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "min_base_price": base_price.min_base_price,
            "max_base_price": base_price.max_base_price,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(base_price)),
        Ok(_) => Err("BasePrice not found.".to_string()),
        Err(err) => Err(format!("Failed to update base_price: {err}")),
    }
}

/// DELETE-эндпоинт для удаления объекта "BasePrice" по ID.
pub async fn delete_base_price(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<BasePrice>("base_prices");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("BasePrice with ID {id} deleted.")),
        Ok(_) => Err("BasePrice not found.".to_string()),
        Err(err) => Err(format!("Failed to delete base_price: {err}")),
    }
}
