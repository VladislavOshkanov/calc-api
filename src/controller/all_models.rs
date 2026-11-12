use crate::AppState;
use crate::error::ApiError;
use crate::model::age_experience::AgeExperience;
use crate::model::base_price::BasePrice;
use crate::model::kbm::Kbm;
use crate::model::limitation::Limitation;
use crate::model::place::Place;
use crate::model::power::Power;
use crate::model::season::Season;
use axum::{Json, extract::State};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct AllModelsResponse {
    pub age_experience: Vec<AgeExperience>,
    pub kbm: Vec<Kbm>,
    pub limitation: Vec<Limitation>,
    pub place: Vec<Place>,
    pub power: Vec<Power>,
    pub season: Vec<Season>,
    pub base_price: Vec<BasePrice>,
}

pub async fn get_all_models(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AllModelsResponse>, ApiError> {
    let mut age_exp_cursor = state
        .database()
        .collection::<AgeExperience>("age_experiences")
        .find(doc! {})
        .await?;
    let mut kbm_cursor = state
        .database()
        .collection::<Kbm>("kbms")
        .find(doc! {})
        .await?;
    let mut limitation_cursor = state
        .database()
        .collection::<Limitation>("limitations")
        .find(doc! {})
        .await?;
    let mut place_cursor = state
        .database()
        .collection::<Place>("places")
        .find(doc! {})
        .await?;
    let mut power_cursor = state
        .database()
        .collection::<Power>("powers")
        .find(doc! {})
        .await?;
    let mut season_cursor = state
        .database()
        .collection::<Season>("seasons")
        .find(doc! {})
        .await?;
    let mut base_price_cursor = state
        .database()
        .collection::<BasePrice>("base_prices")
        .find(doc! {})
        .await?;

    let mut age_experience = Vec::new();
    let mut kbm = Vec::new();
    let mut limitation = Vec::new();
    let mut place = Vec::new();
    let mut power = Vec::new();
    let mut season = Vec::new();
    let mut base_price = Vec::new();

    while let Some(doc) = age_exp_cursor.try_next().await? {
        age_experience.push(doc);
    }
    while let Some(doc) = kbm_cursor.try_next().await? {
        kbm.push(doc);
    }
    while let Some(doc) = limitation_cursor.try_next().await? {
        limitation.push(doc);
    }
    while let Some(doc) = place_cursor.try_next().await? {
        place.push(doc);
    }
    while let Some(doc) = power_cursor.try_next().await? {
        power.push(doc);
    }
    while let Some(doc) = season_cursor.try_next().await? {
        season.push(doc);
    }
    while let Some(doc) = base_price_cursor.try_next().await? {
        base_price.push(doc);
    }

    Ok(Json(AllModelsResponse {
        age_experience,
        kbm,
        limitation,
        place,
        power,
        season,
        base_price,
    }))
}
