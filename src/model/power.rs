use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// Модель Power для коэффициента мощности автомобиля.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Power {
    pub id: Option<ObjectId>, // Уникальный идентификатор в MongoDB
    pub min_power: i32,       // Минимальная мощность в л.с.
    pub max_power: i32,       // Максимальная мощность в л.с.
    pub coefficent: f64,      // Коэффициент для диапазона мощности
}