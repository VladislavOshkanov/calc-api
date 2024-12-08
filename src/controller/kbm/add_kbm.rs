use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::model::Kbm;
use crate::DB;

/// POST-эндпоинт для добавления нового объекта "Kbm".
#[post("/admin/kbm", data = "<kbm>")]
pub async fn add_kbm(db: Connection<DB>, kbm: Json<Kbm>) -> Result<Json<Kbm>, String> {
    let collection = db.database("openapi").collection::<Kbm>("kbms");

    let result = collection
        .insert_one(kbm.clone().into_inner(), None)
        .await;

    match result {
        Ok(insert_result) => {
            info!("Kbm saved with ID: {:?}", insert_result.inserted_id);
            Ok(kbm)
        }
        Err(err) => {
            warn!("Unable to save kbm: {err}");
            Err(format!("Failed to add kbm: {err}"))
        }
    }
}