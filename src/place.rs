use rocket::serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// Структура данных для представления места.
///
/// Используется для сериализации/десериализации JSON-данных,
/// передаваемых через API.
///
/// # Поля:
/// - `name`: Название места (строка, ссылающаяся на входные данные).
/// - `coefficent`: Коэффициент, связанный с местом (вещественное число).

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Place {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] // Связь с MongoDB `_id`
    pub id: Option<ObjectId>, // ID объекта
    pub name: String,         // Название
    pub coefficent: f64,      // Коэффициент
}