use anyhow::{Context, Result};
use mongodb::Client;
use mongodb::bson::DateTime;
use mongodb::options::ClientOptions;
use openapi::model::age_experience::AgeExperience;
use openapi::model::base_price::BasePrice;
use openapi::model::kbm::Kbm;
use openapi::model::limitation::Limitation;
use openapi::model::power::Power;
use openapi::model::season::Season;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct ReferenceDataFile {
    base_price: BasePriceSeed,
    limitation: Vec<LimitationSeed>,
    power: Vec<PowerSeed>,
    season: Vec<SeasonSeed>,
    kbm: Vec<KbmSeed>,
    age_experience: Vec<AgeExperienceSeed>,
}

#[derive(Deserialize)]
struct BasePriceSeed {
    min_base_price: f64,
    max_base_price: f64,
}

#[derive(Deserialize)]
struct LimitationSeed {
    limited: bool,
    coefficient: f64,
}

#[derive(Deserialize)]
struct PowerSeed {
    min_power: i32,
    max_power: i32,
    coefficient: f64,
}

#[derive(Deserialize)]
struct SeasonSeed {
    months: u32,
    coefficient: f64,
}

#[derive(Deserialize)]
struct KbmSeed {
    class: i32,
    coefficient: f64,
}

#[derive(Deserialize)]
struct AgeExperienceSeed {
    age: u32,
    experience: u32,
    coefficient: f64,
    label: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = "data/osago_reference_data.json";
    let mongo_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let raw = fs::read_to_string(Path::new(file_path))
        .with_context(|| format!("failed to read {}", file_path))?;
    let payload: ReferenceDataFile =
        serde_json::from_str(&raw).context("failed to parse osago reference data json")?;

    let options = ClientOptions::parse(&mongo_uri).await?;
    let client = Client::with_options(options)?;
    let db = client.database("openapi");

    db.collection::<Limitation>("limitations")
        .delete_many(mongodb::bson::doc! {})
        .await?;
    db.collection::<Power>("powers")
        .delete_many(mongodb::bson::doc! {})
        .await?;
    db.collection::<Season>("seasons")
        .delete_many(mongodb::bson::doc! {})
        .await?;
    db.collection::<Kbm>("kbms")
        .delete_many(mongodb::bson::doc! {})
        .await?;
    db.collection::<AgeExperience>("age_experiences")
        .delete_many(mongodb::bson::doc! {})
        .await?;
    db.collection::<BasePrice>("base_prices")
        .delete_many(mongodb::bson::doc! {})
        .await?;

    let limitations: Vec<Limitation> = payload
        .limitation
        .into_iter()
        .map(|item| Limitation {
            id: None,
            limited: item.limited,
            coefficient: item.coefficient,
        })
        .collect();
    db.collection::<Limitation>("limitations")
        .insert_many(limitations)
        .await?;

    let powers: Vec<Power> = payload
        .power
        .into_iter()
        .map(|item| Power {
            id: None,
            min_power: item.min_power,
            max_power: item.max_power,
            coefficient: item.coefficient,
        })
        .collect();
    db.collection::<Power>("powers").insert_many(powers).await?;

    let seasons: Vec<Season> = payload
        .season
        .into_iter()
        .map(|item| Season {
            id: None,
            months: item.months,
            coefficient: item.coefficient,
        })
        .collect();
    db.collection::<Season>("seasons")
        .insert_many(seasons)
        .await?;

    let kbms: Vec<Kbm> = payload
        .kbm
        .into_iter()
        .map(|item| Kbm {
            id: None,
            class: item.class,
            coefficient: item.coefficient,
        })
        .collect();
    db.collection::<Kbm>("kbms").insert_many(kbms).await?;

    let age_experiences: Vec<AgeExperience> = payload
        .age_experience
        .into_iter()
        .map(|item| AgeExperience {
            id: None,
            age: item.age,
            experience: item.experience,
            coefficient: item.coefficient,
            label: item.label,
        })
        .collect();
    db.collection::<AgeExperience>("age_experiences")
        .insert_many(age_experiences)
        .await?;

    let base_price = BasePrice {
        id: None,
        min_base_price: payload.base_price.min_base_price,
        max_base_price: payload.base_price.max_base_price,
        created_at: DateTime::now(),
    };
    db.collection::<BasePrice>("base_prices")
        .insert_one(base_price)
        .await?;

    println!("Seeded reference OSAGO data from {}", file_path);
    println!("Territory coefficients are still loaded via cargo run --bin seed_places");

    Ok(())
}
