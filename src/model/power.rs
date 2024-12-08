use rocket::serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// Модель Power для коэффициента мощности автомобиля.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Power {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Уникальный идентификатор в MongoDB
    pub min_power: i32,       // Минимальная мощность в л.с.
    pub max_power: i32,       // Максимальная мощность в л.с.
    pub coefficent: f64,      // Коэффициент для диапазона мощности
}