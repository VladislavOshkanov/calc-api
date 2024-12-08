use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::{doc, oid::ObjectId};
use crate::model::kbm::Kbm;
use crate::DB;

/// GET-эндпоинт для получения одного объекта "Kbm" по ID.
#[get("/admin/kbm/<id>")]
pub async fn get_kbm(db: Connection<DB>, id: String) -> Result<Json<Kbm>, String> {
    let collection = db.database("openapi").collection::<Kbm>("kbms");

    let object_id = ObjectId::parse_str(&id).map_err(|err| format!("Invalid ID format: {err}"))?;

    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(kbm)) => Ok(Json(kbm)),
        Ok(None) => Err("Kbm not found".to_string()),
        Err(err) => Err(format!("Failed to fetch kbm: {err}")),
    }
}