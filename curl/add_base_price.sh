#!/bin/bash

# Пример добавления новой базовой цены
curl -X POST http://localhost:8000/admin/base_price \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ADMIN_TOKEN" \
  -d '{
    "min_base_price": 5000.0,
    "max_base_price": 15000.0
  }'

echo ""
