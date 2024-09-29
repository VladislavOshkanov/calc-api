use rocket::serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Place<'r> {
    pub name: &'r str,
    pub coefficent: f64
}

