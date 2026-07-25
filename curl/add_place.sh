curl -v -X POST http://127.0.0.1:8000/admin/place \
    -H "Authorization: Bearer test_admin_token" \
    -H "Content-Type: application/json" \
    -d '{ "name" : "Novosibirsk", "coefficient": 1.44 }'
