use rocket_db_pools::Connection;
use mongodb::bson::oid::ObjectId;
use crate::model::Place;
use mongodb::bson::doc;
use crate::DB;

/// DELETE-эндпоинт для удаления объекта "Place" по ID.
#[delete("/admin/place/<id>")]
pub async fn delete_place(db: Connection<DB>, id: String) -> Result<String, String> {
    let collection = db.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(delete_result) if delete_result.deleted_count > 0 => {
            Ok(format!("Place with ID {id} deleted"))
        }
        Ok(_) => Err("Place not found".to_string()),
        Err(err) => Err(format!("Failed to delete place: {err}")),
    }
}