use crate::model::Power;
use crate::DB;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use futures::TryStreamExt;
use mongodb::bson::doc;


/// Получить все коэффициенты мощности.
#[get("/admin/power")]
pub async fn get_powers(db: Connection<DB>) -> Result<Json<Vec<Power>>, String> {
    let collection = db.database("openapi").collection::<Power>("power");

    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|err| format!("Failed to execute find query: {err}"))?;

    let mut powers = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|err| format!("Error fetching document: {err}"))? {
        powers.push(doc);
    }

    Ok(Json(powers))
}