use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use crate::model::Place;
use crate::DB;

/// GET-эндпоинт для получения одного объекта "Place" по ID.
#[get("/admin/place/<id>")]
pub async fn get_place(db: Connection<DB>, id: String) -> Result<Json<Place>, String> {
    let collection = db.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };

    match collection.find_one(filter, None).await {
        Ok(Some(place)) => Ok(Json(place)),
        Ok(None) => Err("Place not found".to_string()),
        Err(err) => Err(format!("Failed to fetch place: {err}")),
    }
}