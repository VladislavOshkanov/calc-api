---
id: setup
title: Локальный запуск
---

# Локальный запуск

## Требования

- Rust toolchain
- MongoDB
- Node.js и npm только если нужно запускать сайт документации

## Запуск сервиса

### 1. Поднять MongoDB

```bash
docker run -d --name openapi-mongo -p 27017:27017 mongo:7
```

### 2. Заполнить справочники

```bash
cargo run --bin seed_reference_data
cargo run --bin seed_places
```

### 3. Запустить сервер

```bash
cargo run --bin openapi
```

Сервис по умолчанию слушает `127.0.0.1:8000` и подключается к `mongodb://localhost:27017`.

### 4. Проверить интерфейсы

- `http://127.0.0.1:8000/`
- `http://127.0.0.1:8000/admin`

## Локальный запуск документации

Перейдите в каталог `docs` и установите зависимости:

```bash
cd docs
npm install
npm run start
```

Для production-сборки:

```bash
npm run build
```

## Проверка проекта

```bash
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```
