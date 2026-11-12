use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::kbm::{CreateKbm, Kbm};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

/// POST-эндпоинт для добавления нового объекта "KBM".
pub async fn add_kbm(
    State(state): State<Arc<AppState>>,
    ApiJson(create_kbm): ApiJson<CreateKbm>,
) -> Result<Json<Kbm>, ApiError> {
    create_kbm.validate()?;
    let collection = state.database().collection("kbms");

    let kbm = Kbm {
        id: None,
        coefficient: create_kbm.coefficient,
        class: create_kbm.class,
    };

    let result = collection.insert_one(kbm.clone()).await?;

    let mut kbm_with_id = kbm;
    kbm_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(kbm_with_id))
}

/// GET-эндпоинт для получения списка всех объектов "Kbm".
pub async fn get_kbms(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Kbm>>, ApiError> {
    let collection = state.database().collection::<Kbm>("kbms");

    let mut cursor = collection.find(doc! {}).await?;

    let mut kbms = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        kbms.push(doc);
    }

    Ok(Json(kbms))
}

/// GET-эндпоинт для получения объекта "Kbm" по ID.
pub async fn get_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Kbm>, ApiError> {
    let collection = state.database().collection::<Kbm>("kbms");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(kbm)) => Ok(Json(kbm)),
        Ok(None) => Err(ApiError::not_found("Kbm not found.")),
        Err(err) => Err(err.into()),
    }
}

/// PUT-эндпоинт для обновления объекта "Kbm" по ID.
pub async fn update_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(kbm): ApiJson<Kbm>,
) -> Result<Json<Kbm>, ApiError> {
    kbm.validate()?;
    let collection = state.database().collection::<Kbm>("kbms");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "coefficient": kbm.coefficient,
            "class": kbm.class,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(kbm)),
        Ok(_) => Err(ApiError::not_found("Kbm not found.")),
        Err(err) => Err(err.into()),
    }
}

/// DELETE-эндпоинт для удаления объекта "Kbm" по ID.
pub async fn delete_kbm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<Kbm>("kbms");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Kbm with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("Kbm not found.")),
        Err(err) => Err(err.into()),
    }
}
