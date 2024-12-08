use crate::model::Power;
use crate::DB;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::{doc, oid::ObjectId};

/// Обновить коэффициент мощности по ID.
#[put("/admin/power/<id>", data = "<power>")]
pub async fn update_power(
    db: Connection<DB>,
    id: String,
    power: Json<Power>,
) -> Result<Json<Power>, String> {
    let collection = db.database("openapi").collection::<Power>("power");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "min_power": power.min_power,
            "max_power": power.max_power,
            "coefficent": power.coefficent,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(update_result) if update_result.matched_count > 0 => Ok(power),
        Ok(_) => Err("Power not found".to_string()),
        Err(err) => Err(format!("Failed to update power: {err}")),
    }
}