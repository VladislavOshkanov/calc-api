use rocket_db_pools::Connection;
use crate::model::kbm::Kbm;
use mongodb::bson::{doc, oid::ObjectId};
use crate::DB;

/// DELETE-эндпоинт для удаления объекта "Kbm" по ID.
#[delete("/admin/kbm/<id>")]
pub async fn delete_kbm(db: Connection<DB>, id: String) -> Result<String, String> {
    let collection = db.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|err| format!("Invalid ID format: {err}"))?;

    let filter = doc! { "_id": object_id };

    match collection.delete_one(filter, None).await {
        Ok(delete_result) if delete_result.deleted_count > 0 => {
            Ok(format!("Kbm with ID {id} deleted"))
        }
        Ok(_) => Err("Kbm not found".to_string()),
        Err(err) => Err(format!("Failed to delete kbm: {err}")),
    }
}