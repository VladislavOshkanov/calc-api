# ОСАГО API

Axum-сервис для расчёта стоимости ОСАГО с публичным калькулятором, отдельной админ-панелью и MongoDB в роли хранилища справочников.

## Что есть в проекте

- `GET /` — публичный калькулятор.
- `GET /admin` — HTML-интерфейс администратора.
- `GET /api/all-models` — загрузка всех справочников одним запросом.
- `POST /api/calculate-coefficient` — расчёт итогового коэффициента и диапазона цены.
- `/admin/*` — CRUD API для справочников `Place`, `Power`, `Kbm`, `AgeExperience`, `Season`, `Limitation` и `BasePrice`.
- `seed_reference_data` и `seed_places` — сидеры для начального наполнения данных.

## Быстрый старт

### Требования

- Rust toolchain
- MongoDB

### Поднять MongoDB

```bash
docker run -d --name openapi-mongo -p 27017:27017 mongo:7
```

Основной сервис по умолчанию подключается к `mongodb://localhost:27017` и использует базу `openapi`.

### Загрузить справочники

Сначала загрузите типовые коэффициенты и базовый тариф:

```bash
cargo run --bin seed_reference_data
```

Затем загрузите территориальные коэффициенты из `data/place.html`:

```bash
cargo run --bin seed_places
```

`seed_reference_data` очищает коллекции `limitations`, `powers`, `seasons`, `kbms`, `age_experiences` и `base_prices` перед вставкой. `seed_places` делает upsert по полю `name`.

### Запустить сервис

```bash
cargo run --bin openapi
```

После запуска доступны:

- `http://127.0.0.1:8000/`
- `http://127.0.0.1:8000/admin`

## Переменные окружения

- `ADMIN_TOKEN` — Bearer-токен для всех административных API под `/admin/*`. Если переменная не задана, используется `test_admin_token`.
- `MONGODB_URI` — используется сидерами `seed_reference_data` и `seed_places`.

Основной сервер сейчас не читает `MONGODB_URI` и всегда подключается к `mongodb://localhost:27017`.

## Логика расчёта

Формула расчёта:

`Базовый тариф × КТ × КБМ × КО × КВС × КМ × КС`

Где:

- `КТ` — `Place`
- `КБМ` — `Kbm`
- `КО` — `Limitation`
- `КВС` — `AgeExperience`
- `КМ` — `Power`
- `КС` — `Season`

Особенности:

- если `limitation.limited = false`, сервис принудительно использует `КБМ = 1.0` и `КВС = 1.0`;
- если `season.months >= 9`, сервис принудительно использует `КС = 1.0`;
- базовый тариф берётся из самой свежей записи `BasePrice` по `created_at`;
- в `Place` и `Power` используется только поле `coefficient`.

## API

### Публичный расчёт

`POST /api/calculate-coefficient`

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

Пример ответа:

```json
{
  "total_coefficient": 1.8,
  "min_price": 9000.0,
  "max_price": 12600.0,
  "coefficients": {
    "age_experience": 1.08,
    "kbm": 1.17,
    "limitation": 1.0,
    "place": 1.64,
    "power": 1.2,
    "season": 0.8
  }
}
```

### Админские маршруты

Все API-маршруты под `/admin/*`, включая `GET`, требуют заголовок:

```http
Authorization: Bearer <ADMIN_TOKEN>
```

Поддерживаются ресурсы:

- `/admin/place`
- `/admin/power`
- `/admin/kbm`
- `/admin/age_experience`
- `/admin/season`
- `/admin/limitation`
- `/admin/base_price`

## Документация

Исходники документации на Docusaurus лежат в `docs/docs`.

Локальный запуск:

```bash
cd docs
npm install
npm run start
```

Node.js нужен только для сайта документации, не для самого API.

## Проверка проекта

```bash
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

E2E-тесты запускаются против уже поднятого сервера:

```bash
E2E_BASE_URL=http://127.0.0.1:8000 ADMIN_TOKEN=test_admin_token cargo test --test e2e_tests
```
