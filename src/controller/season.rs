use crate::AppState;
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
    Json(create_season): Json<CreateSeason>,
) -> Json<Season> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("seasons");

    let season = Season {
        id: None,
        months: create_season.months,
        coefficient: create_season.coefficient,
    };

    let result = collection
        .insert_one(season.clone())
        .await
        .expect("Failed to insert season.");

    let mut season_with_id = season;
    season_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(season_with_id)
}

pub async fn get_seasons(State(state): State<Arc<AppState>>) -> Json<Vec<Season>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Season>("seasons");

    let mut cursor = collection
        .find(doc! {})
        .await
        .expect("Failed to fetch seasons");

    let mut seasons = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        seasons.push(doc);
    }

    Json(seasons)
}

pub async fn get_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Season>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Season>("seasons");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(season)) => Ok(Json(season)),
        Ok(None) => Err("Season not found".to_string()),
        Err(err) => Err(format!("Failed to fetch season: {err}")),
    }
}

pub async fn update_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(season): Json<Season>,
) -> Result<Json<Season>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Season>("seasons");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "months": season.months,
            "coefficient": season.coefficient,
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(season)),
        Ok(_) => Err("Season not found.".to_string()),
        Err(err) => Err(format!("Failed to update season: {err}")),
    }
}

pub async fn delete_season(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Season>("seasons");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Season with ID {id} deleted.")),
        Ok(_) => Err("Season not found.".to_string()),
        Err(err) => Err(format!("Failed to delete season: {err}")),
    }
}
