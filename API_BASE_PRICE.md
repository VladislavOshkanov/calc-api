# API для управления базовыми ценами (BasePrice)

## Описание

Модель `BasePrice` предназначена для хранения базовых цен страховки с минимальным и максимальным значениями. Эти цены используются в расчетах итоговой стоимости страховки.

## Структура данных

```rust
pub struct BasePrice {
    pub id: Option<ObjectId>,     // Уникальный идентификатор в MongoDB
    pub min_base_price: f64,      // Минимальная базовая цена
    pub max_base_price: f64,      // Максимальная базовая цена
    pub created_at: DateTime,     // Дата и время создания записи
}
```

## Endpoints

### 1. Добавление новой базовой цены

```
POST /admin/base_price
```

#### Входные данные
```json
{
  "min_base_price": 5000.0,
  "max_base_price": 15000.0
}
```

#### Ответ (200 OK)
```json
{
  "_id": "507f1f77bcf86cd799439017",
  "min_base_price": 5000.0,
  "max_base_price": 15000.0,
  "created_at": "2024-01-15T10:30:00.000Z"
}
```

### 2. Получение списка всех базовых цен

```
GET /admin/base_price
```

#### Ответ (200 OK)
```json
[
  {
    "_id": "507f1f77bcf86cd799439017",
    "min_base_price": 5000.0,
    "max_base_price": 15000.0
  },
  {
    "_id": "507f1f77bcf86cd799439018",
    "min_base_price": 7000.0,
    "max_base_price": 20000.0
  }
]
```

### 3. Получение базовой цены по ID

```
GET /admin/base_price/:id
```

#### Ответ (200 OK)
```json
{
  "_id": "507f1f77bcf86cd799439017",
  "min_base_price": 5000.0,
  "max_base_price": 15000.0
}
```

#### Ошибки
- `400 Bad Request` - неверный формат ID
- `404 Not Found` - запись не найдена

### 4. Обновление базовой цены

```
PUT /admin/base_price/:id
```

#### Входные данные
```json
{
  "_id": "507f1f77bcf86cd799439017",
  "min_base_price": 6000.0,
  "max_base_price": 18000.0
}
```

#### Ответ (200 OK)
```json
{
  "_id": "507f1f77bcf86cd799439017",
  "min_base_price": 6000.0,
  "max_base_price": 18000.0
}
```

### 5. Удаление базовой цены

```
DELETE /admin/base_price/:id
```

#### Ответ (200 OK)
```
BasePrice with ID 507f1f77bcf86cd799439017 deleted.
```

## Аутентификация

Все endpoints требуют токен администратора в заголовке:
```
Authorization: Bearer YOUR_ADMIN_TOKEN
```

## Примеры использования

### Добавление новой базовой цены
```bash
curl -X POST http://localhost:8000/admin/base_price \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "min_base_price": 5000.0,
    "max_base_price": 15000.0
  }'
```

### Получение всех базовых цен
```bash
curl -X GET http://localhost:8000/admin/base_price \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN"
```

### Обновление базовой цены
```bash
curl -X PUT http://localhost:8000/admin/base_price/507f1f77bcf86cd799439017 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "_id": "507f1f77bcf86cd799439017",
    "min_base_price": 6000.0,
    "max_base_price": 18000.0
  }'
```

### Удаление базовой цены
```bash
curl -X DELETE http://localhost:8000/admin/base_price/507f1f77bcf86cd799439017 \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN"
```

## Использование в расчетах

BasePrice используется в контроллере `coefficient_calculator` для расчета итоговых цен страховки. Система автоматически использует последнюю добавленную базовую цену (по дате создания):

- `min_price = total_coefficient * min_base_price`
- `max_price = total_coefficient * max_base_price`

Где `total_coefficient` - произведение всех коэффициентов из других моделей, а базовые цены берутся из самой последней записи BasePrice.

## Валидация

- `min_base_price` и `max_base_price` должны быть положительными числами
- `min_base_price` не должно превышать `max_base_price`
- ID должен быть валидным ObjectId MongoDB

## Ошибки

### 400 Bad Request
```json
{
  "error": "Invalid ID format"
}
```

### 404 Not Found
```json
{
  "error": "BasePrice not found."
}
```

### 500 Internal Server Error
```json
{
  "error": "Failed to fetch base_price: connection error"
}
```
