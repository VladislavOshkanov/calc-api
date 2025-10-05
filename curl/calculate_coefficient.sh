#!/bin/bash

# Пример использования API для расчета произведения коэффициентов
# Замените ID на реальные ID из вашей базы данных

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

echo ""
