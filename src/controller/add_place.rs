use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::place::Place;
use crate::DB;

/// POST-эндпоинт для добавления нового объекта "Place".
#[post("/admin/place", data = "<place>")]
pub async fn add_place(db: Connection<DB>, place: Json<Place>) -> Result<Json<Place>, String> {
    let collection = db.database("openapi").collection::<Place>("places");

    let result = collection
        .insert_one(place.clone().into_inner(), None)
        .await;

    match result {
        Ok(insert_result) => {
            info!("Place saved with ID: {:?}", insert_result.inserted_id);
            Ok(place)
        }
        Err(err) => {
            warn!("Unable to save place: {err}");
            Err(format!("Failed to add place: {err}"))
        }
    }
}