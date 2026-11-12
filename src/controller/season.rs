use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::season::{CreateSeason, Season};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

pub async fn add_season(
    State(state): State<Arc<AppState>>,
    ApiJson(create_season): ApiJson<CreateSeason>,
) -> Result<Json<Season>, ApiError> {
    create_season.validate()?;
    let collection = state.database().collection("seasons");

    let season = Season {
        id: None,
        months: create_season.months,
        coefficient: create_season.coefficient,
    };

    let result = collection.insert_one(season.clone()).await?;

    let mut season_with_id = season;
    season_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(season_with_id))
}

pub async fn get_seasons(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Season>>, ApiError> {
    let collection = state.database().collection::<Season>("seasons");

    let mut cursor = collection.find(doc! {}).await?;

    let mut seasons = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        seasons.push(doc);
    }

    Ok(Json(seasons))
}

pub async fn get_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Season>, ApiError> {
    let collection = state.database().collection::<Season>("seasons");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(season)) => Ok(Json(season)),
        Ok(None) => Err(ApiError::not_found("Season not found")),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(season): ApiJson<Season>,
) -> Result<Json<Season>, ApiError> {
    season.validate()?;
    let collection = state.database().collection::<Season>("seasons");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "months": season.months,
            "coefficient": season.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(season)),
        Ok(_) => Err(ApiError::not_found("Season not found.")),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state.database().collection::<Season>("seasons");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Season with ID {id} deleted.")),
        Ok(_) => Err(ApiError::not_found("Season not found.")),
        Err(err) => Err(err.into()),
    }
}
