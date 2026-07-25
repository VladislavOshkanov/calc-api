---
id: api-reference
title: API Reference
---

# API Reference

## Базовые адреса

- Локально: `http://127.0.0.1:8000`
- Публичная страница: `GET /`
- Админ-панель: `GET /admin`

## Публичные маршруты

### GET /api/all-models

Возвращает все справочники сразу:

- `age_experience`
- `kbm`
- `limitation`
- `place`
- `power`
- `season`
- `base_price`

Важно: `base_price` в этом ответе возвращается целиком, без выбора только самой свежей записи.

Пример ответа:

```json
{
  "age_experience": [],
  "kbm": [],
  "limitation": [],
  "place": [],
  "power": [],
  "season": [],
  "base_price": []
}
```

### POST /api/calculate-coefficient

Рассчитывает итоговый коэффициент и цену по выбранным справочникам.

Пример запроса:

```json
{
  "age_experience_id": "ObjectId",
  "kbm_id": "ObjectId",
  "limitation_id": "ObjectId",
  "place_id": "ObjectId",
  "power_id": "ObjectId",
  "season_id": "ObjectId"
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

### Важные правила расчёта

- Если `limitation.limited = false`, то `КБМ = 1.0` и `КВС = 1.0`.
- Если `season.months >= 9`, то `КС = 1.0`.
- Для базовой цены берётся самая свежая запись `BasePrice` по `created_at`.

## Админские маршруты

Все административные маршруты требуют заголовок:

```http
Authorization: Bearer <ADMIN_TOKEN>
```

Это правило относится ко всем `/admin/*` API, включая `GET`.

### Place

- `POST /admin/place`
- `GET /admin/place`
- `GET /admin/place/{id}`
- `PUT /admin/place/{id}`
- `DELETE /admin/place/{id}`

Пример payload:

```json
{
  "name": "Москва",
  "coefficient": 1.64
}
```

### Power

- `POST /admin/power`
- `GET /admin/power`
- `GET /admin/power/{id}`
- `PUT /admin/power/{id}`
- `DELETE /admin/power/{id}`

Пример payload:

```json
{
  "min_power": 100,
  "max_power": 150,
  "coefficient": 1.2
}
```

### Kbm

- `POST /admin/kbm`
- `GET /admin/kbm`
- `GET /admin/kbm/{id}`
- `PUT /admin/kbm/{id}`
- `DELETE /admin/kbm/{id}`

Пример payload:

```json
{
  "class": 3,
  "coefficient": 1.17
}
```

### AgeExperience

- `POST /admin/age_experience`
- `GET /admin/age_experience`
- `GET /admin/age_experience/{id}`
- `PUT /admin/age_experience/{id}`
- `DELETE /admin/age_experience/{id}`

Пример payload:

```json
{
  "age": 30,
  "experience": 10,
  "coefficient": 1.04,
  "label": "30+ лет, стаж 10+"
}
```

### Season

- `POST /admin/season`
- `GET /admin/season`
- `GET /admin/season/{id}`
- `PUT /admin/season/{id}`
- `DELETE /admin/season/{id}`

Пример payload:

```json
{
  "months": 6,
  "coefficient": 0.8
}
```

### Limitation

- `POST /admin/limitation`
- `GET /admin/limitation`
- `GET /admin/limitation/{id}`
- `PUT /admin/limitation/{id}`
- `DELETE /admin/limitation/{id}`

Пример payload:

```json
{
  "limited": true,
  "coefficient": 1.0
}
```

### BasePrice

- `POST /admin/base_price`
- `GET /admin/base_price`
- `GET /admin/base_price/{id}`
- `PUT /admin/base_price/{id}`
- `DELETE /admin/base_price/{id}`

Пример payload для `POST`:

```json
{
  "min_base_price": 1646.0,
  "max_base_price": 7535.0
}
```

## Форматы ошибок

Если ресурс не найден или ID некорректен, обработчики возвращают текстовую ошибку и подходящий HTTP-статус. Для авторизации используется `401 Unauthorized`.
