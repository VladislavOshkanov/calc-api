use crate::model::age_experience::AgeExperience;
use crate::model::base_price::BasePrice;
use crate::model::kbm::Kbm;
use crate::model::limitation::Limitation;
use crate::model::place::Place;
use crate::model::power::Power;
use crate::model::season::Season;
use crate::AppState;
use axum::{extract::State, Json};
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

pub async fn get_all_models(State(state): State<Arc<AppState>>) -> Json<AllModelsResponse> {
    let client = &state.db_client;
    let mut age_exp_cursor = client
        .database("openapi")
        .collection::<AgeExperience>("age_experiences")
        .find(doc! {})
        .await
        .expect("Failed to fetch age_experiences");
    let mut kbm_cursor = client
        .database("openapi")
        .collection::<Kbm>("kbms")
        .find(doc! {})
        .await
        .expect("Failed to fetch kbms");
    let mut limitation_cursor = client
        .database("openapi")
        .collection::<Limitation>("limitations")
        .find(doc! {})
        .await
        .expect("Failed to fetch limitations");
    let mut place_cursor = client
        .database("openapi")
        .collection::<Place>("places")
        .find(doc! {})
        .await
        .expect("Failed to fetch places");
    let mut power_cursor = client
        .database("openapi")
        .collection::<Power>("powers")
        .find(doc! {})
        .await
        .expect("Failed to fetch powers");
    let mut season_cursor = client
        .database("openapi")
        .collection::<Season>("seasons")
        .find(doc! {})
        .await
        .expect("Failed to fetch seasons");
    let mut base_price_cursor = client
        .database("openapi")
        .collection::<BasePrice>("base_prices")
        .find(doc! {})
        .await
        .expect("Failed to fetch base_prices");

    let mut age_experience = Vec::new();
    let mut kbm = Vec::new();
    let mut limitation = Vec::new();
    let mut place = Vec::new();
    let mut power = Vec::new();
    let mut season = Vec::new();
    let mut base_price = Vec::new();

    while let Some(doc) = age_exp_cursor.try_next().await.expect("Error fetching doc") {
        age_experience.push(doc);
    }
    while let Some(doc) = kbm_cursor.try_next().await.expect("Error fetching doc") {
        kbm.push(doc);
    }
    while let Some(doc) = limitation_cursor
        .try_next()
        .await
        .expect("Error fetching doc")
    {
        limitation.push(doc);
    }
    while let Some(doc) = place_cursor.try_next().await.expect("Error fetching doc") {
        place.push(doc);
    }
    while let Some(doc) = power_cursor.try_next().await.expect("Error fetching doc") {
        power.push(doc);
    }
    while let Some(doc) = season_cursor.try_next().await.expect("Error fetching doc") {
        season.push(doc);
    }
    while let Some(doc) = base_price_cursor
        .try_next()
        .await
        .expect("Error fetching doc")
    {
        base_price.push(doc);
    }

    Json(AllModelsResponse {
        age_experience,
        kbm,
        limitation,
        place,
        power,
        season,
        base_price,
    })
}
