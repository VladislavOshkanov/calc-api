use axum::{
    extract::State,
    Json,
};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

/// Структура для входных данных с ID всех моделей
#[derive(Deserialize, Debug)]
pub struct CoefficientRequest {
    pub age_experience_id: String,
    pub kbm_id: String,
    pub limitation_id: String,
    pub place_id: String,
    pub power_id: String,
    pub season_id: String,
}

/// Структура для ответа с результатом расчета
#[derive(Serialize, Debug)]
pub struct CoefficientResponse {
    pub total_coefficient: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub coefficients: CoefficientDetails,
}

/// Детали коэффициентов для каждой модели
#[derive(Serialize, Debug)]
pub struct CoefficientDetails {
    pub age_experience: f64,
    pub kbm: f64,
    pub limitation: f64,
    pub place: f64,
    pub power: f64,
    pub season: f64,
}

/// POST-эндпоинт для расчета произведения коэффициентов
pub async fn calculate_coefficient(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CoefficientRequest>,
) -> Result<Json<CoefficientResponse>, String> {
    let client = &state.db_client;

    // Парсинг ID
    let age_experience_id = ObjectId::parse_str(&request.age_experience_id)
        .map_err(|_| "Invalid age_experience_id format".to_string())?;
    let kbm_id = ObjectId::parse_str(&request.kbm_id)
        .map_err(|_| "Invalid kbm_id format".to_string())?;
    let limitation_id = ObjectId::parse_str(&request.limitation_id)
        .map_err(|_| "Invalid limitation_id format".to_string())?;
    let place_id = ObjectId::parse_str(&request.place_id)
        .map_err(|_| "Invalid place_id format".to_string())?;
    let power_id = ObjectId::parse_str(&request.power_id)
        .map_err(|_| "Invalid power_id format".to_string())?;
    let season_id = ObjectId::parse_str(&request.season_id)
        .map_err(|_| "Invalid season_id format".to_string())?;

    // Получение коэффициента из AgeExperience
    let age_experience_collection = client.database("openapi").collection::<serde_json::Value>("age_experiences");
    let age_experience_filter = doc! { "_id": age_experience_id };
    let age_experience_doc = age_experience_collection
        .find_one(age_experience_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch age_experience: {}", e))?
        .ok_or("AgeExperience not found")?;
    let age_experience_coeff = age_experience_doc["coefficient"]
        .as_f64()
        .ok_or("Invalid coefficient in age_experience")?;

    // Получение коэффициента из Kbm
    let kbm_collection = client.database("openapi").collection::<serde_json::Value>("kbms");
    let kbm_filter = doc! { "_id": kbm_id };
    let kbm_doc = kbm_collection
        .find_one(kbm_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch kbm: {}", e))?
        .ok_or("Kbm not found")?;
    let kbm_coeff = kbm_doc["coefficient"]
        .as_f64()
        .ok_or("Invalid coefficient in kbm")?;

    // Получение коэффициента из Limitation
    let limitation_collection = client.database("openapi").collection::<serde_json::Value>("limitations");
    let limitation_filter = doc! { "_id": limitation_id };
    let limitation_doc = limitation_collection
        .find_one(limitation_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch limitation: {}", e))?
        .ok_or("Limitation not found")?;
    let limitation_coeff = limitation_doc["coefficient"]
        .as_f64()
        .ok_or("Invalid coefficient in limitation")?;

    // Получение коэффициента из Place
    let place_collection = client.database("openapi").collection::<serde_json::Value>("places");
    let place_filter = doc! { "_id": place_id };
    let place_doc = place_collection
        .find_one(place_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch place: {}", e))?
        .ok_or("Place not found")?;
    // Some records may have typo 'coefficent', try both keys
    let place_coeff = place_doc.get("coefficient").and_then(|v| v.as_f64())
        .or_else(|| place_doc.get("coefficent").and_then(|v| v.as_f64()))
        .ok_or("Invalid coefficient in place")?;

    // Получение коэффициента из Power
    let power_collection = client.database("openapi").collection::<serde_json::Value>("powers");
    let power_filter = doc! { "_id": power_id };
    let power_doc = power_collection
        .find_one(power_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch power: {}", e))?
        .ok_or("Power not found")?;
    // Support both 'coefficient' and common typo 'coefficent'
    let power_coeff = power_doc.get("coefficient").and_then(|v| v.as_f64())
        .or_else(|| power_doc.get("coefficent").and_then(|v| v.as_f64()))
        .ok_or("Invalid coefficient in power")?;

    // Получение коэффициента из Season
    let season_collection = client.database("openapi").collection::<serde_json::Value>("seasons");
    let season_filter = doc! { "_id": season_id };
    let season_doc = season_collection
        .find_one(season_filter, None)
        .await
        .map_err(|e| format!("Failed to fetch season: {}", e))?
        .ok_or("Season not found")?;
    let season_coeff = season_doc["coefficient"]
        .as_f64()
        .ok_or("Invalid coefficient in season")?;

    // Получение последней базовой цены по дате создания (сортировка по created_at в убывающем порядке)
    let base_price_collection = client.database("openapi").collection::<serde_json::Value>("base_prices");
    let sort_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "created_at": -1 }) // Сортировка по убыванию даты
        .build();
    
    let base_price_doc = base_price_collection
        .find_one(None, sort_options)
        .await
        .map_err(|e| format!("Failed to fetch latest base_price: {}", e))?
        .ok_or("No base_price records found")?;
    let min_base_price = base_price_doc["min_base_price"]
        .as_f64()
        .ok_or("Invalid min_base_price in base_price")?;
    let max_base_price = base_price_doc["max_base_price"]
        .as_f64()
        .ok_or("Invalid max_base_price in base_price")?;

    // Расчет произведения коэффициентов
    let total_coefficient = age_experience_coeff * kbm_coeff * limitation_coeff * place_coeff * power_coeff * season_coeff;
    
    // Расчет итоговых цен (общий коэффициент * базовые цены)
    let min_price = total_coefficient * min_base_price;
    let max_price = total_coefficient * max_base_price;

    let response = CoefficientResponse {
        total_coefficient,
        min_price,
        max_price,
        coefficients: CoefficientDetails {
            age_experience: age_experience_coeff,
            kbm: kbm_coeff,
            limitation: limitation_coeff,
            place: place_coeff,
            power: power_coeff,
            season: season_coeff,
        },
    };

    Ok(Json(response))
}
