#!/usr/bin/env bash

# delete all notes
curl -s -X DELETE http://0.0.0.0:8871/api/v1/notes > /dev/null 2>&1

# login as authorized user
echo -n '{"login":"bob", "password":"bob123"}' > data.json
LOGIN_RESULT=$(curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:8871/api/v1/login)
TOKEN=${LOGIN_RESULT:18:36}
AUTH_HEADER="Authorization: Bearer $TOKEN"
JSON_HEADER="Content-Type: application/json"

# create a new note
echo -n '{"title":"Note1","content":"Content1"}' > data.json
NOTE=$(curl -s -d '@data.json' -H "$AUTH_HEADER" -H "$JSON_HEADER" -X POST http://0.0.0.0:8871/api/v1/notes)
NOTE_ID=${NOTE:15:36}

# retrieve a note without authorization
curl -s "http://0.0.0.0:8871/api/v1/notes/$NOTE_ID"

# delete data file
rm data.json
