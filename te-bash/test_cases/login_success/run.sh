#!/usr/bin/env bash

# prepare JSON data file containing login and password
echo -n '{"login":"bob", "password":"bob123"}' >> data.json

# call login endpoint and save the JSON response in a variable
A=$(curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:8871/api/v1/login)

# test if the response matches expected result
if [[ "$A" =~ ^\{\"data\":\{\"token\":\"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\"\}\}$ ]]; then
  echo "YES"
else
  echo "NO"
fi

# delete data file
rm data.json

