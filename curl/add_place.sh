curl -v -X POST http://localhost:8000/admin/place \
    -H "Authorization: Bearer super-secret-token" \
    -H "Content-Type: application/json" \
    -d '{ "name" : "Novosibirsk", "coefficent": 1.44 }'