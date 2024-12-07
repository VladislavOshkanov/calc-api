use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use futures::stream::TryStreamExt;
use crate::place::Place;
use crate::DB;

/// GET-эндпоинт для получения списка всех объектов "Place".
#[get("/admin/place")]
pub async fn get_places(db: Connection<DB>) -> Result<Json<Vec<Place>>, String> {
    let collection = db.database("openapi").collection::<Place>("places");

    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|err| format!("Failed to execute find query: {err}"))?;

    let mut places = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|err| format!("Error fetching document: {err}"))? {
        places.push(Place {
            id: doc.id,
            name: doc.name.to_owned(),
            coefficent: doc.coefficent,
        });
    }

    Ok(Json(places))
}