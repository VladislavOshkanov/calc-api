---
id: models
title: Модели данных
---

# Модели данных

Ниже перечислены основные Mongo-модели из `src/model/*` и коллекции, в которых они хранятся.

## Place

Коллекция: `places`

Поля:

- `id: ObjectId`
- `name: String`
- `coefficient: f64`

## Power

Коллекция: `powers`

Поля:

- `id: ObjectId`
- `min_power: i32`
- `max_power: i32`
- `coefficient: f64`

## Kbm

Коллекция: `kbms`

Поля:

- `id: ObjectId`
- `coefficient: f64`
- `class: i32`

## AgeExperience

Коллекция: `age_experiences`

Поля:

- `id: ObjectId`
- `age: u32`
- `experience: u32`
- `coefficient: f64`
- `label: Option<String>`

## Season

Коллекция: `seasons`

Поля:

- `id: ObjectId`
- `months: u32`
- `coefficient: f64`

## Limitation

Коллекция: `limitations`

Поля:

- `id: ObjectId`
- `limited: bool`
- `coefficient: f64`

## BasePrice

Коллекция: `base_prices`

Поля:

- `id: ObjectId`
- `min_base_price: f64`
- `max_base_price: f64`
- `created_at: DateTime`

## Модели создания

Для каждого ресурса есть отдельная структура `Create*`, которая используется в `POST`-маршрутах и не содержит `_id`.

Исключения и детали:

- `CreateBasePrice` не принимает `created_at`: это поле выставляется на сервере при создании записи.
- `CreateAgeExperience` поддерживает необязательное поле `label`.
