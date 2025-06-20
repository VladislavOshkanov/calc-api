# ОСАГО API

REST API для расчёта стоимости ОСАГО на основе базовой ставки и коэффициентов (место, мощность, КБМ).

## Возможности
- CRUD для мест, мощностей, КБМ
- Авторизация для админских маршрутов (Bearer токен)

## Быстрый старт
1. Установите [Rust](https://www.rust-lang.org/) и MongoDB
2. Клонируйте репозиторий и соберите проект:
   ```bash
   git clone https://github.com/VladislavOshkanov/calc-api.git
   cd calc-api
   cargo build
   ```
3. Запустите сервер:
   ```bash
   cargo run
   ```
   По умолчанию сервер слушает порт 3000.

## Пример запроса
Добавить место (требуется авторизация):
```
POST /admin/place
Authorization: Bearer <token>
{
  "name": "Москва",
  "coefficent": 1.5
}
```

## Лицензия
MIT License