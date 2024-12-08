use crate::model::Power;
use crate::DB;
use rocket_db_pools::Connection;
use mongodb::bson::{doc, oid::ObjectId};


/// Удалить коэффициент мощности по ID.
#[delete("/admin/power/<id>")]
pub async fn delete_power(db: Connection<DB>, id: String) -> Result<String, String> {
    let collection = db.database("openapi").collection::<Power>("power");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(delete_result) if delete_result.deleted_count > 0 => {
            Ok(format!("Power with ID {id} deleted"))
        }
        Ok(_) => Err("Power not found".to_string()),
        Err(err) => Err(format!("Failed to delete power: {err}")),
    }
}