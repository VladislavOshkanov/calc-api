use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Модель Power для коэффициента мощности автомобиля.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Power {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Уникальный идентификатор в MongoDB
    pub min_power: i32,  // Минимальная мощность в л.с.
    pub max_power: i32,  // Максимальная мощность в л.с.
    pub coefficent: f64, // Коэффициент для диапазона мощности
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePower {
    pub min_power: i32,  // Минимальная мощность в л.с.
    pub max_power: i32,  // Максимальная мощность в л.с.
    pub coefficent: f64, // Коэффициент для диапазона мощности
}
