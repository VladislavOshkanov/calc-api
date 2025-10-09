use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Season {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub months: u32,
    pub coefficient: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSeason {
    pub months: u32,
    pub coefficient: f64,
}
