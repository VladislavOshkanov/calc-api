use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::limitation::{CreateLimitation, Limitation};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

pub async fn add_limitation(
    State(state): State<Arc<AppState>>,
    ApiJson(create_limitation): ApiJson<CreateLimitation>,
) -> Result<Json<Limitation>, ApiError> {
    create_limitation.validate()?;
    let collection = state.database().collection("limitations");

    let limitation = Limitation {
        id: None,
        limited: create_limitation.limited,
        coefficient: create_limitation.coefficient,
    };

    let result = collection.insert_one(limitation.clone()).await?;

    let mut limitation_with_id = limitation;
    limitation_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(limitation_with_id))
}

pub async fn get_limitations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Limitation>>, ApiError> {
    let collection = state.database().collection::<Limitation>("limitations");

    let mut cursor = collection.find(doc! {}).await?;

    let mut limitations = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        limitations.push(doc);
    }

    Ok(Json(limitations))
}

pub async fn get_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Limitation>, ApiError> {
    let collection = state.database().collection::<Limitation>("limitations");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(limitation)) => Ok(Json(limitation)),
        Ok(None) => Err(ApiError::not_found("Limitation not found")),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(limitation): ApiJson<Limitation>,
) -> Result<Json<Limitation>, ApiError> {
    limitation.validate()?;
    let collection = state.database().collection::<Limitation>("limitations");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "limited": limitation.limited,
            "coefficient": limitation.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(limitation)),
        Ok(_) => Err(ApiError::not_found("Limitation not found.")),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<Limitation>("limitations");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Limitation with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("Limitation not found.")),
        Err(err) => Err(err.into()),
    }
}
