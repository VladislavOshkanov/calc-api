use crate::model::Power;
use crate::DB;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::{doc, oid::ObjectId};

/// Получить коэффициент мощности по ID.
#[get("/admin/power/<id>")]
pub async fn get_power(db: Connection<DB>, id: String) -> Result<Json<Power>, String> {
    let collection = db.database("openapi").collection::<Power>("power");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(power)) => Ok(Json(power)),
        Ok(None) => Err("Power not found".to_string()),
        Err(err) => Err(format!("Failed to fetch power: {err}")),
    }
}