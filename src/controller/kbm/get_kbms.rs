use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::model::kbm::Kbm;
use futures::TryStreamExt;
use crate::DB;

/// GET-эндпоинт для получения списка всех объектов "Kbm".
#[get("/admin/kbm")]
pub async fn get_kbms(db: Connection<DB>) -> Result<Json<Vec<Kbm>>, String> {
    let collection = db.database("openapi").collection::<Kbm>("kbms");

    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|err| format!("Failed to execute find query: {err}"))?;

    let mut kbms = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(|err| format!("Error fetching document: {err}"))? {
        kbms.push(Kbm {
            id: doc.id,
            coefficient: doc.coefficient,
            class: doc.class,
        });
    }

    Ok(Json(kbms))
}