use crate::model::Power;
use crate::DB;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use mongodb::bson::doc;

/// Создать новый коэффициент мощности.
#[post("/admin/power", data = "<power>")]
pub async fn add_power(db: Connection<DB>, power: Json<Power>) -> Result<Json<Power>, String> {
    let collection = db.database("openapi").collection::<Power>("power");

    let result = collection
        .insert_one(power.clone().into_inner(), None)
        .await;

    match result {
        Ok(insert_result) => {
            info!("Power saved with ID: {:?}", insert_result.inserted_id);
            Ok(power)
        }
        Err(err) => {
            warn!("Unable to save power: {err}");
            Err(format!("Failed to add power: {err}"))
        }
    }
}