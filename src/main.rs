mod place;
mod controller;

use controller::add_place::add_place;
use controller::get_places::get_places;
use controller::get_place::get_place;
use controller::update_place::update_place;
use controller::delete_place::delete_place;

#[macro_use] extern crate rocket;

use rocket_db_pools::Database;
use rocket_db_pools::mongodb;

/// Определение базы данных Rocket с использованием MongoDB.
#[derive(Database)]
#[database("openapi_mongo")]
struct DB(mongodb::Client);

/// Главная функция запуска приложения.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("Starting Rocket application...");

    let _rocket = rocket::build()
        .attach(DB::init())
        .mount(
            "/",
            routes![
                add_place,
                get_places,
                get_place,
                update_place,
                delete_place
            ],
        )
        .launch()
        .await?;

    Ok(())
}