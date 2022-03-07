#!/usr/bin/env bash

# delete all notes
curl -s -X DELETE http://0.0.0.0:8871/api/v1/notes > /dev/null 2>&1

# login as authorized user
echo -n '{"login":"bob", "password":"bob123"}' > data.json
LOGIN_RESULT=$(curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:8871/api/v1/login)
TOKEN=${LOGIN_RESULT:18:36}
AUTH_HEADER="Authorization: Bearer $TOKEN"
JSON_HEADER="Content-Type: application/json"

# create a new NOTE
echo -n '{"title":"Note1","content":"Content1"}' > data.json
NOTE=$(curl -s -d '@data.json' -H "$AUTH_HEADER" -H "$JSON_HEADER" -X POST http://0.0.0.0:8871/api/v1/notes)
NOTE_ID=${NOTE:19:36}

# retrieve a NOTE with specified identifier
NOTE_DETAILS=$(curl -s -H "$AUTH_HEADER" "http://0.0.0.0:8871/api/v1/notes/$NOTE_ID")

# test if the response matches expected result
if [[ "$NOTE_DETAILS" =~ ^\{\"data\":\{\"noteId\":\"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\",\"title\":\"Note1\",\"content\":\"Content1\"\}\}$ ]]; then
  echo "YES"
else
  echo "NO"
fi

# delete data file
rm data.json
