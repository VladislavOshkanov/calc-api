use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

/// Модель BasePrice для базовых цен страховки
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasePrice {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Уникальный идентификатор в MongoDB
    pub min_base_price: f64,  // Минимальная базовая цена
    pub max_base_price: f64,  // Максимальная базовая цена
    pub created_at: DateTime, // Дата и время создания записи
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateBasePrice {
    pub min_base_price: f64, // Минимальная базовая цена
    pub max_base_price: f64, // Максимальная базовая цена
}
