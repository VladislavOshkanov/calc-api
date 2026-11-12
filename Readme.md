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

- Rust 1.99.1 (версия закреплена в `rust-toolchain.toml`)
- MongoDB

### Поднять MongoDB

```bash
docker run -d --name openapi-mongo -p 27017:27017 -v openapi_mongo_data:/data/db mongo:8.0.15
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
export ADMIN_TOKEN="$(openssl rand -hex 32)"
cargo run --bin openapi
```

После запуска доступны:

- `http://127.0.0.1:8000/`
- `http://127.0.0.1:8000/admin`

## Переменные окружения

- `ADMIN_TOKEN` — Bearer-токен для всех административных API под `/admin/*`. Обязателен: при отсутствии или пустом значении сервер не запускается.
- `MONGODB_URI` — адрес MongoDB для сервера и сидеров, по умолчанию `mongodb://localhost:27017`.

Сервер использует базу из `MONGODB_URI`, если она указана, иначе `openapi`. Сидеры заполняют базу `openapi`.

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

Node.js нужен для сайта документации и теста интерфейса, не для самого API.

## Проверка проекта

```bash
cargo test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

E2E-тесты запускаются против уже поднятого сервера:

```bash
E2E_BASE_URL=http://127.0.0.1:8000 cargo test --test e2e_tests -- --ignored
```

`ADMIN_TOKEN` при запуске E2E должен совпадать с токеном сервера. Используйте отдельную тестовую базу в `MONGODB_URI`. Без `--ignored` E2E явно помечаются как пропущенные.

Проверка расчёта с локальной MongoDB создаёт и удаляет собственную временную базу:

```bash
cargo test --test regression_test -- --include-ignored
node --test tests/admin_ui_test.cjs
```

Ошибки API возвращаются в JSON `{"message":"..."}` со статусами 400 (неверный ID), 401 (авторизация), 404 (запись отсутствует), 422 (невалидные данные), 500 (ошибка данных сервера) и 503 (ошибка MongoDB). Цены и коэффициенты должны быть конечными положительными числами; границы диапазонов проверяются при создании и обновлении.

Статические ресурсы встроены в бинарный файл; после изменения JS/CSS требуется пересборка.

## Обновление стека

Сборка, Docker и CI используют Rust 1.99.1. Для локальной работы через rustup запускайте `cargo` из `$HOME/.cargo/bin`, чтобы применялся `rust-toolchain.toml`; Homebrew Cargo использует установленный Homebrew Rust независимо от этого файла.

HTTP-сервер запускается через `axum::serve` и корректно завершает активные запросы при SIGTERM или Ctrl+C. Документация использует Docusaurus 3.10.2; CI собирает её на Node.js 24.

В Compose закреплена MongoDB 8.0.15 и объявлен volume `mongo_data`. Перед переводом существующего контейнера на этот volume перенесите данные из текущего хранилища и проверьте резервную копию. Запуск Compose с новым пустым volume не переносит существующие данные автоматически. Обновление Rust-зависимостей не требует миграции коллекций или перехода на BSON 3.

Для совместимости со старыми справочниками `Place` и `Power` читают поле `coefficent`; API возвращает каноническое `coefficient`. При обновлении записи старое поле удаляется, поэтому массовая миграция не требуется.

В `docs/package.json` заданы security overrides для `tinypool`, `serialize-javascript`, `qs` и `uuid` внутри `sockjs`. Они закрывают уязвимости транзитивных зависимостей, которые не устраняются обычным `npm audit fix`. В цепочке SVGR остаётся предупреждение низкой важности для `deepmerge` без исправленного релиза; проверяйте его при следующих обновлениях.
