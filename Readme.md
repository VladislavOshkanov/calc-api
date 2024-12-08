# ОСАГО API

Это REST API для расчёта стоимости ОСАГО, которое опирается на базовую ставку и различные коэффициенты. API предоставляет функциональность для работы с данными, такими как добавление, обновление, удаление и получение информации о местах, коэффициентах мощности и коэффициентах для классификации автомобилей.

## Основные фичи

- **Модели**: Место (Place), Мощность (Power), КБМ (Kbm).
- **CRUD-операции** для работы с данными:
  - Добавление, получение, обновление и удаление мест, мощностей и КБМ.
- **Авторизация**:
  - Для всех админских маршрутов требуется авторизация с использованием Bearer токена.

## Запуск проекта

### Требования

- [Rust](https://www.rust-lang.org/)
- MongoDB (или MongoDB Atlas)

### Установка

1. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/your-repo/osa-api.git
   cd osa-api
   ```

2. Настройте MongoDB:
   - Убедитесь, что у вас установлен MongoDB, либо используйте MongoDB Atlas.
   - Создайте базу данных `openapi`.

3. Установите зависимости:
   ```bash
   cargo build
   ```

### Запуск сервера

Для запуска сервера используйте команду:

```bash
cargo run
```

Сервер будет слушать на порту 3000 по умолчанию.

## API документация

### 1. **POST /admin/place**
Добавляет новое место.

**Тело запроса**:
```json
{
    "name": "Название места",
    "coefficent": 1.5
}
```

**Ответ**:
```json
{
    "name": "Название места",
    "coefficent": 1.5
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 2. **GET /admin/place**
Получает все места.

**Ответ**:
```json
[
    {
        "name": "Название места 1",
        "coefficent": 1.5
    },
    {
        "name": "Название места 2",
        "coefficent": 1.7
    }
]
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 3. **GET /admin/place/{id}**
Получает информацию о месте по его ID.

**Ответ**:
```json
{
    "name": "Название места",
    "coefficent": 1.5
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 4. **PUT /admin/place/{id}**
Обновляет информацию о месте по его ID.

**Тело запроса**:
```json
{
    "name": "Новое название места",
    "coefficent": 2.0
}
```

**Ответ**:
```json
{
    "name": "Новое название места",
    "coefficent": 2.0
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 5. **DELETE /admin/place/{id}**
Удаляет место по его ID.

**Ответ**:
```json
{
    "message": "Place with ID {id} deleted."
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 6. **POST /admin/power**
Добавляет новый диапазон мощности.

**Тело запроса**:
```json
{
    "min_power": 100,
    "max_power": 200,
    "coefficent": 1.2
}
```

**Ответ**:
```json
{
    "min_power": 100,
    "max_power": 200,
    "coefficent": 1.2
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 7. **GET /admin/power**
Получает все диапазоны мощности.

**Ответ**:
```json
[
    {
        "min_power": 100,
        "max_power": 200,
        "coefficent": 1.2
    },
    {
        "min_power": 200,
        "max_power": 300,
        "coefficent": 1.5
    }
]
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 8. **POST /admin/kbm**
Добавляет новый КБМ.

**Тело запроса**:
```json
{
    "coefficient": 1.0,
    "class": 1
}
```

**Ответ**:
```json
{
    "coefficient": 1.0,
    "class": 1
}
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

### 9. **GET /admin/kbm**
Получает все КБМ.

**Ответ**:
```json
[
    {
        "coefficient": 1.0,
        "class": 1
    },
    {
        "coefficient": 1.5,
        "class": 2
    }
]
```

**Авторизация**:
- Требуется заголовок `Authorization: Bearer secret-token`.

## Тестирование с помощью `curl`

Примеры запросов с использованием `curl`:

### Добавить место:
```bash
curl -X POST http://localhost:3000/admin/place \
    -H "Authorization: Bearer secret-token" \
    -d '{"name": "Test Place", "coefficent": 1.5}'
```

### Получить все места:
```bash
curl -X GET http://localhost:3000/admin/place \
    -H "Authorization: Bearer secret-token"
```

### Получить место по ID:
```bash
curl -X GET http://localhost:3000/admin/place/{id} \
    -H "Authorization: Bearer secret-token"
```

## Авторизация

Для доступа к админским маршрутам (начинающимся с `/admin/`) необходимо передавать заголовок `Authorization` с токеном:

```bash
Authorization: Bearer secret-token
```

## Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для подробностей.
```

Этот `README.md` содержит:

- Общее описание проекта и его назначения.
- Шаги по установке и запуску.
- Примеры API-запросов с использованием `curl`.
- Информацию об авторизации для админских эндпоинтов.

Не забудьте заменить `secret-token` на фактический токен, который вы используете для авторизации.