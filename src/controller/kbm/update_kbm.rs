use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::{doc, oid::ObjectId};
use crate::model::kbm::Kbm;
use crate::DB;

/// PUT-эндпоинт для обновления объекта "Kbm" по ID.
#[put("/admin/kbm/<id>", data = "<kbm>")]
pub async fn update_kbm(db: Connection<DB>, id: String, kbm: Json<Kbm>) -> Result<Json<Kbm>, String> {
    let collection = db.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|err| format!("Invalid ID format: {err}"))?;

    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "coefficient": kbm.coefficient,
            "class": kbm.class,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(update_result) if update_result.matched_count > 0 => Ok(kbm),
        Ok(_) => Err("Kbm not found".to_string()),
        Err(err) => Err(format!("Failed to update kbm: {err}")),
    }
}