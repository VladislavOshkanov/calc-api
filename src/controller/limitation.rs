use axum::{Json, extract::{Path, State}};
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;
use crate::{AppState};
use crate::model::limitation::{Limitation, CreateLimitation};
use futures::TryStreamExt;

pub async fn add_limitation(
    State(state): State<Arc<AppState>>,
    Json(create_limitation): Json<CreateLimitation>,
) -> Json<Limitation> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("limitations");

    let limitation = Limitation {
        id: None,
        limited: create_limitation.limited,
        coefficient: create_limitation.coefficient,
    };

    let result = collection
        .insert_one(limitation.clone(), None)
        .await
        .expect("Failed to insert limitation.");

    let mut limitation_with_id = limitation;
    limitation_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(limitation_with_id)
}

pub async fn get_limitations(State(state): State<Arc<AppState>>) -> Json<Vec<Limitation>> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Limitation>("limitations");

    let mut cursor = collection.find(None, None).await.expect("Failed to fetch limitations");

    let mut limitations = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        limitations.push(doc);
    }

    Json(limitations)
}

pub async fn get_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Limitation>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Limitation>("limitations");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(limitation)) => Ok(Json(limitation)),
        Ok(None) => Err("Limitation not found".to_string()),
        Err(err) => Err(format!("Failed to fetch limitation: {err}")),
    }
}

pub async fn update_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(limitation): Json<Limitation>,
) -> Result<Json<Limitation>, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Limitation>("limitations");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "limited": limitation.limited,
            "coefficient": limitation.coefficient,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(limitation)),
        Ok(_) => Err("Limitation not found.".to_string()),
        Err(err) => Err(format!("Failed to update limitation: {err}")),
    }
}

pub async fn delete_limitation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection::<Limitation>("limitations");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => Ok(format!("Limitation with ID {id} deleted.")),
        Ok(_) => Err("Limitation not found.".to_string()),
        Err(err) => Err(format!("Failed to delete limitation: {err}")),
    }
} 