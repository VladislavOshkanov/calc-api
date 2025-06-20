use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Kbm {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // ID объекта
    pub coefficient: f64,     // Коэффициент (дробное число)
    pub class: i32,           // Класс (целое число)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateKbm {
    pub coefficient: f64,     // Коэффициент (дробное число)
    pub class: i32,           // Класс (целое число)
}