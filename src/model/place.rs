use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Place {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // ID объекта
    pub name: String, // Название места
    #[serde(alias = "coefficient")]
    pub coefficent: f64, // Коэффициент
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreatePlace {
    pub name: String, // Название места
    #[serde(alias = "coefficient")]
    pub coefficent: f64, // Коэффициент
}
