# ОСАГО API

REST API и веб-интерфейс для расчёта стоимости ОСАГО на основе базовой ставки и коэффициентов.

## Краткое описание

Проект предоставляет:
- CRUD-эндпоинты для управления справочными моделями (Place, Power, KBM, AgeExperience, Season, Limitation, BasePrice).
- Публичный фронтенд для расчёта стоимости ОСАГО (`/`, `static/index.html`).
- Отдельный админский фронтенд (`/admin`, `static/admin.html`).
- Административные маршруты под префиксом /admin/*, защищённые Bearer-токеном.
- Эндпоинт расчёта коэффициента: POST /api/calculate-coefficient.
- Сидеры для загрузки типовых коэффициентов ОСАГО и территориальных коэффициентов.

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

Формула: Базовый тариф × КТ (коэффициент территории) × КБМ (коэффициент бонус-малус) × КО (коэффициент ограничения) × КВС (коэффициент возраста и стажа) × КМ (коэффициент мощности) × КС (коэффициент сезонности).

Особые правила:
- Если в полисе нет ограничения по водителям (limitation.limited = false):
  - КО берётся из справочника Limitation (для физлиц обычно 3.16)
  - КБМ принудительно принимается равным 1.0
  - КВС принудительно принимается равным 1.0
- КС: при периоде использования от 9 до 12 месяцев коэффициент сезонности равен 1.0

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

Логика: сервер получает все указанные записи из БД, перемножает их коэффициенты и умножает на min_base_price/max_base_price из самой новой записи BasePrice (по полю created_at).

## Админ-панель (веб)

В проекте есть отдельный фронтенд для администратора, расположенный в `static/admin.html` и доступный по `/admin`. Он позволяет:
- Ввести Bearer-токен (Admin Token)
- Выбрать тип модели
- Просмотреть список записей
- Создать/обновить/удалить запись через JSON-пейлоад
- Подставить шаблон payload для нужной модели
- Кликнуть по существующей записи и сразу перенести её в форму редактирования

Панель использует существующие маршруты /admin/* и требует валидного токена.

Публичный калькулятор расположен в `static/index.html` и больше не смешан с админскими действиями.

## Наполнение коэффициентов

В репозитории есть два сценария загрузки справочников:

1. Базовые коэффициенты ОСАГО, КБМ, КВС, КО, КС, диапазоны мощности и базовый тариф:

```bash
cargo run --bin seed_reference_data
```

Источник данных хранится в `data/osago_reference_data.json`.

2. Территориальные коэффициенты:

```bash
cargo run --bin seed_places
```

Скрипт парсит HTML-снимок из `data/place.html` и загружает его в коллекцию `places`.

## Запуск и разработка

### Требования
- Rust toolchain
- MongoDB

### Docker MongoDB без пароля

```bash
docker run -d --name openapi-mongo -p 27017:27017 mongo:7
```

Контейнер запускает MongoDB без авторизации. Подключение по строке mongodb://localhost:27017 доступно из хоста благодаря пробросу порта.

После завершения работы:

```bash
docker stop openapi-mongo && docker rm openapi-mongo
```

### Сборка и запуск

```bash
git clone https://github.com/VladislavOshkanov/calc-api.git
cd calc-api
cargo build
cargo run --bin openapi
```

Сервер по умолчанию слушает 127.0.0.1:8000, а клиент подключается к MongoDB на localhost:27017.

## Тесты и форматирование

- Запуск тестов: cargo test
- Форматирование: cargo fmt
- Статический анализ: cargo clippy --all-targets --all-features -- -D warnings

### Интеграционные тесты

1. Убедитесь, что MongoDB и сервер запущены.
2. Выполните:

```bash
E2E_BASE_URL=http://127.0.0.1:8000 ADMIN_TOKEN=test_admin_token cargo test --test e2e_tests
```

3. Если переопределяете `ADMIN_TOKEN` или `E2E_BASE_URL`, используйте одинаковые значения для сервера и тестов.

## Изменение логики расчёта

- Поле выбора базовой цены удалено из пользовательского интерфейса.
- При расчёте используется последняя запись `BasePrice` из коллекции `base_prices`.
- Минимальная и максимальная цены считаются как произведение суммарного коэффициента на `min_base_price` и `max_base_price` из этой записи.
- Пользовательский и админский интерфейсы разведены по разным страницам.
