mod place;

use rocket::serde::json::Json;
use tokio::sync::Mutex;
use crate::place::Place;
use rocket::State;

#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::mongodb::{self, Client};


#[derive(Database)]
#[database("openapi_mongo")]
struct DB(mongodb::Client);

#[post("/admin/place", data = "<place>")]
async fn add_place(db: Connection<DB>, place: Json<Place<'_>>) -> Json<Place<'_>> {
    
    db.database("openapi")
        .collection::<Place>("places")
        .insert_one(place.clone().into_inner(), None)
        .await;


    Json(Place {name: place.name, coefficent: place.coefficent})
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Hello");

    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        .attach(DB::init())
        .mount("/", routes![add_place])
        .launch()
        .await?;

    Ok(())
}



