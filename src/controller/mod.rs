// Подключение модулей контроллеров
pub mod place;
pub mod power;
pub mod kbm;

// Экспорт часто используемых маршрутов (опционально)
pub use place::*;
pub use power::*;
pub use kbm::*;