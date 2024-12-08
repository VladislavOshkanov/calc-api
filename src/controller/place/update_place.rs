use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use crate::model::Place;
use crate::DB;

/// PUT-эндпоинт для обновления объекта "Place" по ID.
#[put("/admin/place/<id>", data = "<place>")]
pub async fn update_place(
    db: Connection<DB>,
    id: String,
    place: Json<Place>,
) -> Result<Json<Place>, String> {
    let collection = db.database("openapi").collection::<Place>("places");

    let object_id = ObjectId::parse_str(&id).map_err(|err| {
        warn!("Invalid ID format: {err}");
        format!("Invalid ID format: {err}")
    })?;

    let filter = doc! { "_id": object_id };
    let update = doc! {
        "$set": {
            "name": place.name.clone(),
            "coefficent": place.coefficent,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(update_result) if update_result.matched_count > 0 => Ok(place),
        Ok(_) => Err("Place not found".to_string()),
        Err(err) => Err(format!("Failed to update place: {err}")),
    }
}