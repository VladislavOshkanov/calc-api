use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgeExperience {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub age: u32,
    pub experience: u32,
    pub coefficient: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAgeExperience {
    pub age: u32,
    pub experience: u32,
    pub coefficient: f64,
}
