use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Place {
    pub id: Option<ObjectId>, // ID объекта
    pub name: String,         // Название места
    pub coefficent: f64,      // Коэффициент
}