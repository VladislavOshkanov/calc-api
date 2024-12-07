curl -v -X POST http://localhost:8000/admin/place \
    -H "Content-Type: application/json" \
    -d '{ "name" : "Krasnoyarsk", "coefficent": 1.44 }'