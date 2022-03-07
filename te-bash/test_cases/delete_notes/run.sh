#!/usr/bin/env bash

# create a new note
echo -n '{"title":"Note1","content":"Content1"}' > data.json
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:8871/api/v1/notes > /dev/null 2>&1

# delete all notes
curl -s -X DELETE http://0.0.0.0:8871/api/v1/notes
