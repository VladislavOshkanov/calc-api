use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Kbm {
    pub id: Option<ObjectId>, // ID объекта
    pub coefficient: f64,     // Коэффициент (дробное число)
    pub class: i32,           // Класс (целое число)
}