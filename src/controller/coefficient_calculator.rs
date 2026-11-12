use crate::AppState;
use crate::error::{ApiError, ApiJson};
use axum::{Json, extract::State};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    ApiJson(request): ApiJson<CoefficientRequest>,
) -> Result<Json<CoefficientResponse>, ApiError> {
    // Парсинг ID
    let age_experience_id = ObjectId::parse_str(&request.age_experience_id)
        .map_err(|_| ApiError::bad_request("Invalid age_experience_id format"))?;
    let kbm_id = ObjectId::parse_str(&request.kbm_id)
        .map_err(|_| ApiError::bad_request("Invalid kbm_id format"))?;
    let limitation_id = ObjectId::parse_str(&request.limitation_id)
        .map_err(|_| ApiError::bad_request("Invalid limitation_id format"))?;
    let place_id = ObjectId::parse_str(&request.place_id)
        .map_err(|_| ApiError::bad_request("Invalid place_id format"))?;
    let power_id = ObjectId::parse_str(&request.power_id)
        .map_err(|_| ApiError::bad_request("Invalid power_id format"))?;
    let season_id = ObjectId::parse_str(&request.season_id)
        .map_err(|_| ApiError::bad_request("Invalid season_id format"))?;

    // Получение коэффициента из AgeExperience
    let age_experience_collection = state
        .database()
        .collection::<serde_json::Value>("age_experiences");
    let age_experience_filter = doc! { "_id": age_experience_id };
    let age_experience_doc = age_experience_collection
        .find_one(age_experience_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("AgeExperience not found"))?;
    let age_experience_coeff = age_experience_doc["coefficient"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid coefficient in age_experience"))?;

    // Получение коэффициента из Kbm
    let kbm_collection = state.database().collection::<serde_json::Value>("kbms");
    let kbm_filter = doc! { "_id": kbm_id };
    let kbm_doc = kbm_collection
        .find_one(kbm_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("Kbm not found"))?;
    let kbm_coeff = kbm_doc["coefficient"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid coefficient in kbm"))?;

    // Получение коэффициента из Limitation
    let limitation_collection = state
        .database()
        .collection::<serde_json::Value>("limitations");
    let limitation_filter = doc! { "_id": limitation_id };
    let limitation_doc = limitation_collection
        .find_one(limitation_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("Limitation not found"))?;
    let limitation_coeff = limitation_doc["coefficient"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid coefficient in limitation"))?;

    // Получение коэффициента из Place
    let place_collection = state.database().collection::<serde_json::Value>("places");
    let place_filter = doc! { "_id": place_id };
    let place_doc = place_collection
        .find_one(place_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("Place not found"))?;
    let place_coeff = place_doc
        .get("coefficient")
        .or_else(|| place_doc.get("coefficent"))
        .and_then(|v| v.as_f64())
        .ok_or_else(|| ApiError::internal("Invalid coefficient in place"))?;

    // Получение коэффициента из Power
    let power_collection = state.database().collection::<serde_json::Value>("powers");
    let power_filter = doc! { "_id": power_id };
    let power_doc = power_collection
        .find_one(power_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("Power not found"))?;
    let power_coeff = power_doc
        .get("coefficient")
        .or_else(|| power_doc.get("coefficent"))
        .and_then(|v| v.as_f64())
        .ok_or_else(|| ApiError::internal("Invalid coefficient in power"))?;

    // Получение коэффициента из Season
    let season_collection = state.database().collection::<serde_json::Value>("seasons");
    let season_filter = doc! { "_id": season_id };
    let season_doc = season_collection
        .find_one(season_filter)
        .await?
        .ok_or_else(|| ApiError::not_found("Season not found"))?;
    let mut season_coeff = season_doc["coefficient"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid coefficient in season"))?;

    // Правило КС: при периоде использования >= 9 месяцев коэффициент сезонности равен 1.0
    if let Some(months) = season_doc.get("months").and_then(|v| v.as_i64())
        && months >= 9
    {
        season_coeff = 1.0;
    }

    // Получение последней базовой цены по дате создания (сортировка по created_at в убывающем порядке)
    let base_price_collection = state
        .database()
        .collection::<serde_json::Value>("base_prices");
    let sort_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "created_at": -1 }) // Сортировка по убыванию даты
        .build();

    let base_price_doc = base_price_collection
        .find_one(doc! {})
        .with_options(sort_options)
        .await?
        .ok_or_else(|| ApiError::internal("No base_price records found"))?;
    let min_base_price = base_price_doc["min_base_price"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid min_base_price in base_price"))?;
    let max_base_price = base_price_doc["max_base_price"]
        .as_f64()
        .ok_or_else(|| ApiError::internal("Invalid max_base_price in base_price"))?;

    // Правила для безограничительного полиса:
    // при неограниченном списке водителей КБМ и КВС принимаются равными 1.0.
    let limited_flag = limitation_doc
        .get("limited")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let (eff_kbm_coeff, eff_age_experience_coeff) = if !limited_flag {
        (1.0_f64, 1.0_f64)
    } else {
        (kbm_coeff, age_experience_coeff)
    };

    // Расчет произведения коэффициентов
    let total_coefficient = eff_age_experience_coeff
        * eff_kbm_coeff
        * limitation_coeff
        * place_coeff
        * power_coeff
        * season_coeff;

    // Расчет итоговых цен (общий коэффициент * базовые цены)
    let min_price = total_coefficient * min_base_price;
    let max_price = total_coefficient * max_base_price;

    let response = CoefficientResponse {
        total_coefficient,
        min_price,
        max_price,
        coefficients: CoefficientDetails {
            age_experience: eff_age_experience_coeff,
            kbm: eff_kbm_coeff,
            limitation: limitation_coeff,
            place: place_coeff,
            power: power_coeff,
            season: season_coeff,
        },
    };

    Ok(Json(response))
}
