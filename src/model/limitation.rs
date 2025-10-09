use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Limitation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub limited: bool, // true, если ограниченное число водителей
    pub coefficient: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateLimitation {
    pub limited: bool, // true, если ограниченное число водителей
    pub coefficient: f64,
}
