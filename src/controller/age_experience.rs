use crate::model::age_experience::{AgeExperience, CreateAgeExperience};
use crate::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

pub async fn add_age_experience(
    State(state): State<Arc<AppState>>,
    Json(create_age_experience): Json<CreateAgeExperience>,
) -> Json<AgeExperience> {
    let client = &state.db_client;
    let collection = client.database("openapi").collection("age_experiences");

    let age_experience = AgeExperience {
        id: None,
        age: create_age_experience.age,
        experience: create_age_experience.experience,
        coefficient: create_age_experience.coefficient,
    };

    let result = collection
        .insert_one(age_experience.clone(), None)
        .await
        .expect("Failed to insert age_experience.");

    let mut age_experience_with_id = age_experience;
    age_experience_with_id.id = Some(result.inserted_id.as_object_id().unwrap());

    Json(age_experience_with_id)
}

pub async fn get_age_experiences(State(state): State<Arc<AppState>>) -> Json<Vec<AgeExperience>> {
    let client = &state.db_client;
    let collection = client
        .database("openapi")
        .collection::<AgeExperience>("age_experiences");

    let mut cursor = collection
        .find(None, None)
        .await
        .expect("Failed to fetch age_experiences");

    let mut age_experiences = Vec::new();
    while let Some(doc) = cursor.try_next().await.expect("Error fetching document") {
        age_experiences.push(doc);
    }

    Json(age_experiences)
}

pub async fn get_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<AgeExperience>, String> {
    let client = &state.db_client;
    let collection = client
        .database("openapi")
        .collection::<AgeExperience>("age_experiences");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(age_experience)) => Ok(Json(age_experience)),
        Ok(None) => Err("AgeExperience not found".to_string()),
        Err(err) => Err(format!("Failed to fetch age_experience: {err}")),
    }
}

pub async fn update_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(age_experience): Json<AgeExperience>,
) -> Result<Json<AgeExperience>, String> {
    let client = &state.db_client;
    let collection = client
        .database("openapi")
        .collection::<AgeExperience>("age_experiences");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "age": age_experience.age,
            "experience": age_experience.experience,
            "coefficient": age_experience.coefficient,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(age_experience)),
        Ok(_) => Err("AgeExperience not found.".to_string()),
        Err(err) => Err(format!("Failed to update age_experience: {err}")),
    }
}

pub async fn delete_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, String> {
    let client = &state.db_client;
    let collection = client
        .database("openapi")
        .collection::<AgeExperience>("age_experiences");

    let object_id = ObjectId::parse_str(&id).map_err(|_| "Invalid ID format".to_string())?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => {
            Ok(format!("AgeExperience with ID {id} deleted."))
        }
        Ok(_) => Err("AgeExperience not found.".to_string()),
        Err(err) => Err(format!("Failed to delete age_experience: {err}")),
    }
}
