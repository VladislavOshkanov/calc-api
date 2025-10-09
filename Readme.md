# ОСАГО API

REST API для расчёта стоимости ОСАГО на основе базовой ставки и коэффициентов (место, мощность, КБМ и т.д.).

## Краткое описание

Проект предоставляет:
- CRUD-эндпоинты для управления справочными моделями (Place, Power, KBM, AgeExperience, Season, Limitation, BasePrice).
- Публичный фронтенд для расчёта стоимости ОСАГО (static/index.html).
- Административные маршруты под префиксом /admin/*, защищённые Bearer-токеном.
- Эндпоинт расчёта коэффициента: POST /api/calculate-coefficient.

## Модели

Ниже перечислены модели и их поля (см. src/model/*):

- Place
  - id: ObjectId
  - name: String — название места
  - coefficent: f64 — коэффициент
  - Создание: POST /admin/place
  - Получение списка: GET /admin/place
  - Получение одного: GET /admin/place/:id
  - Обновление: PUT /admin/place/:id
  - Удаление: DELETE /admin/place/:id

- Power
  - id: ObjectId
  - min_power: i32 — минимальная мощность (л.с.)
  - max_power: i32 — максимальная мощность (л.с.)
  - coefficent: f64 — коэффициент для диапазона мощности
  - CRUD: /admin/power

- Kbm
  - id: ObjectId
  - coefficient: f64 — коэффициент КБМ
  - class: i32 — класс
  - CRUD: /admin/kbm

- AgeExperience
  - id: ObjectId
  - age: u32 — возраст водителя
  - experience: u32 — стаж (лет)
  - coefficient: f64
  - CRUD: /admin/age_experience

- Season
  - id: ObjectId
  - months: u32 — число месяцев использования
  - coefficient: f64
  - CRUD: /admin/season

- Limitation
  - id: ObjectId
  - limited: bool — ограниченное число водителей
  - coefficient: f64
  - CRUD: /admin/limitation

- BasePrice
  - id: ObjectId
  - min_base_price: f64
  - max_base_price: f64
  - created_at: DateTime
  - CRUD: /admin/base_price

## API для расчёта стоимости

Эндпоинт: POST /api/calculate-coefficient

Описание: принимает идентификаторы выбранных опций и возвращает суммарный коэффициент и рассчитанные min/max цены на основе последней записи BasePrice.

Пример запроса (Content-Type: application/json):

```json
{
  "age_experience_id": "<ObjectId>",
  "kbm_id": "<ObjectId>",
  "limitation_id": "<ObjectId>",
  "place_id": "<ObjectId>",
  "power_id": "<ObjectId>",
  "season_id": "<ObjectId>"
}
```

Пример ответа (200 OK):

```json
{
  "coefficient": 1.8,
  "min_price": 9000.0,
  "max_price": 12600.0
}
```

Логика: сервер получает все указанные записи из БД, перемножает их коэффициенты и умножает на min_base_price/max_base_price из самой новой записи BasePrice (по полю created_at).

## Админ-панель (веб)

В проекте есть простой фронтенд для администратора, расположенный в static/index.html. Он добавляет секцию "Admin Panel", где можно:
- Ввести Bearer-токен (Admin Token)
- Выбрать тип модели
- Просмотреть список записей
- Создать/обновить/удалить запись через JSON-пейлоад

Панель использует существующие маршруты /admin/* и требует валидного токена.

## Запуск и разработка

### Требования
- Rust toolchain
- MongoDB

### Сборка и запуск

```bash
git clone https://github.com/VladislavOshkanov/calc-api.git
cd calc-api
cargo build
cargo run
```

Сервер по умолчанию слушает 127.0.0.1:8000.

## Тесты и форматирование

- Запуск тестов: cargo test
- Форматирование: cargo fmt
- Статический анализ: cargo clippy --all-targets --all-features -- -D warnings

## Изменение логики расчёта (обновление 2025)

- Поле выбора базовой цены удалено из веб-интерфейса.
- При расчёте используется последняя запись BasePrice из коллекции base_prices.
- Минимальная и максимальная цены считаются как произведение суммарного коэффициента на min_base_price и max_base_price из этой записи.

Если нужно, могу дополнить README примерами API-запросов для каждой модели и добавить пример curl-скриптов. Перечислите, какие именно примеры вы хотите видеть — и я добавлю их.