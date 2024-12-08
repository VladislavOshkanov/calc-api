mod controller;
mod model;

#[macro_use] extern crate rocket;

use rocket_db_pools::Database;
use rocket_db_pools::mongodb;

use crate::controller::add_place::add_place;
use crate::controller::delete_place::delete_place;
use crate::controller::get_place::get_place;
use crate::controller::get_places::get_places;
use crate::controller::update_place::update_place;

use crate::controller::add_kbm::add_kbm;
use crate::controller::delete_kbm::delete_kbm;
use crate::controller::get_kbm::get_kbm;
use crate::controller::get_kbms::get_kbms;
use crate::controller::update_kbm::update_kbm;

use crate::controller::add_power::add_power;
use crate::controller::delete_power::delete_power;
use crate::controller::get_power::get_power;
use crate::controller::get_powers::get_powers;
use crate::controller::update_power::update_power;

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
                // Place endpoints
                add_place, get_places, get_place, update_place, delete_place,
                // Power endpoints
                add_power, get_powers, get_power, update_power, delete_power,
                // KBM endpoints
                add_kbm, get_kbms, get_kbm, update_kbm, delete_kbm
            ],
        )
        .launch()
        .await?;

    Ok(())
}