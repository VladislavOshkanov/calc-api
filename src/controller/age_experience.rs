use crate::AppState;
use crate::error::{ApiError, ApiJson};
use crate::model::Validate;
use crate::model::age_experience::{AgeExperience, CreateAgeExperience};
use axum::{
    Json,
    extract::{Path, State},
};
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::sync::Arc;

pub async fn add_age_experience(
    State(state): State<Arc<AppState>>,
    ApiJson(create_age_experience): ApiJson<CreateAgeExperience>,
) -> Result<Json<AgeExperience>, ApiError> {
    create_age_experience.validate()?;
    let collection = state.database().collection("age_experiences");

    let age_experience = AgeExperience {
        id: None,
        age: create_age_experience.age,
        experience: create_age_experience.experience,
        coefficient: create_age_experience.coefficient,
        label: create_age_experience.label,
    };

    let result = collection.insert_one(age_experience.clone()).await?;

    let mut age_experience_with_id = age_experience;
    age_experience_with_id.id = Some(
        result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| ApiError::internal("Invalid inserted ID"))?,
    );

    Ok(Json(age_experience_with_id))
}

pub async fn get_age_experiences(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AgeExperience>>, ApiError> {
    let collection = state
        .database()
        .collection::<AgeExperience>("age_experiences");

    let mut cursor = collection.find(doc! {}).await?;

    let mut age_experiences = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        age_experiences.push(doc);
    }

    Ok(Json(age_experiences))
}

pub async fn get_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<AgeExperience>, ApiError> {
    let collection = state
        .database()
        .collection::<AgeExperience>("age_experiences");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = mongodb::bson::doc! { "_id": object_id };

    match collection.find_one(filter).await {
        Ok(Some(age_experience)) => Ok(Json(age_experience)),
        Ok(None) => Err(ApiError::not_found("AgeExperience not found")),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ApiJson(age_experience): ApiJson<AgeExperience>,
) -> Result<Json<AgeExperience>, ApiError> {
    age_experience.validate()?;
    let collection = state
        .database()
        .collection::<AgeExperience>("age_experiences");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "age": age_experience.age,
            "experience": age_experience.experience,
            "coefficient": age_experience.coefficient,
            "label": age_experience.label.clone(),
        }
    };

    match collection.update_one(filter, update).await {
        Ok(result) if result.matched_count > 0 => Ok(Json(age_experience)),
        Ok(_) => Err(ApiError::not_found("AgeExperience not found.")),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_age_experience(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<String, ApiError> {
    let collection = state
        .database()
        .collection::<AgeExperience>("age_experiences");

    let object_id =
        ObjectId::parse_str(&id).map_err(|_| ApiError::bad_request("Invalid ID format"))?;
    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter).await {
        Ok(result) if result.deleted_count > 0 => {
            Ok(format!("AgeExperience with ID {id} deleted."))
        }
        Ok(_) => Err(ApiError::not_found("AgeExperience not found.")),
        Err(err) => Err(err.into()),
    }
}
