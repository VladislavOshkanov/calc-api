# API для расчета произведения коэффициентов

## Описание

Новый контроллер `coefficient_calculator` позволяет рассчитать произведение коэффициентов из всех моделей системы:
- AgeExperience (возраст и опыт водителя)
- Kbm (коэффициент бонус-малус)
- Limitation (ограничение количества водителей)
- Place (место регистрации)
- Power (мощность автомобиля)
- Season (сезонность использования)
- BasePrice (базовые цены страховки)

## Endpoint

```
POST /admin/calculate-coefficient
```

## Аутентификация

Требуется токен администратора в заголовке `Authorization: Bearer YOUR_ADMIN_TOKEN`

## Входные данные

```json
{
  "age_experience_id": "507f1f77bcf86cd799439011",
  "kbm_id": "507f1f77bcf86cd799439012",
  "limitation_id": "507f1f77bcf86cd799439013",
  "place_id": "507f1f77bcf86cd799439014",
  "power_id": "507f1f77bcf86cd799439015",
  "season_id": "507f1f77bcf86cd799439016"
}
```

### Поля запроса

- `age_experience_id` (string) - ID записи из коллекции age_experiences
- `kbm_id` (string) - ID записи из коллекции kbms
- `limitation_id` (string) - ID записи из коллекции limitations
- `place_id` (string) - ID записи из коллекции places
- `power_id` (string) - ID записи из коллекции powers
- `season_id` (string) - ID записи из коллекции seasons

## Ответ

### Успешный ответ (200 OK)

```json
{
  "total_coefficient": 2.5,
  "min_price": 12500.0,
  "max_price": 37500.0,
  "coefficients": {
    "age_experience": 1.2,
    "kbm": 0.8,
    "limitation": 1.0,
    "place": 1.5,
    "power": 1.1,
    "season": 1.3
  }
}
```

### Поля ответа

- `total_coefficient` (number) - произведение всех коэффициентов
- `min_price` (number) - минимальная итоговая цена (total_coefficient * min_base_price)
- `max_price` (number) - максимальная итоговая цена (total_coefficient * max_base_price)
- `coefficients` (object) - детализация коэффициентов по каждой модели:
  - `age_experience` (number) - коэффициент возраста и опыта
  - `kbm` (number) - коэффициент бонус-малус
  - `limitation` (number) - коэффициент ограничений
  - `place` (number) - коэффициент места регистрации
  - `power` (number) - коэффициент мощности
  - `season` (number) - коэффициент сезонности

### Ошибки

#### 400 Bad Request
```json
{
  "error": "Invalid age_experience_id format"
}
```

#### 404 Not Found
```json
{
  "error": "AgeExperience not found"
}
```

#### 500 Internal Server Error
```json
{
  "error": "Failed to fetch age_experience: connection error"
}
```

## Пример использования

```bash
curl -X POST http://localhost:8000/admin/calculate-coefficient \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "age_experience_id": "507f1f77bcf86cd799439011",
    "kbm_id": "507f1f77bcf86cd799439012",
    "limitation_id": "507f1f77bcf86cd799439013",
    "place_id": "507f1f77bcf86cd799439014",
    "power_id": "507f1f77bcf86cd799439015",
    "season_id": "507f1f77bcf86cd799439016"
  }'
```

## Реализация

Контроллер находится в файле `src/controller/coefficient_calculator.rs` и включает:

- `CoefficientRequest` - структура для входных данных
- `CoefficientResponse` - структура для ответа
- `CoefficientDetails` - детализация коэффициентов
- `calculate_coefficient` - основная функция обработки запроса

Функция выполняет следующие действия:
1. Парсит все переданные ID
2. Загружает соответствующие записи из MongoDB
3. Извлекает коэффициенты из каждой записи
4. Вычисляет произведение всех коэффициентов
5. Загружает последнюю базовую цену по дате создания из BasePrice
6. Вычисляет итоговые цены (общий коэффициент * базовые цены)
7. Возвращает результат с детализацией и итоговыми ценами
