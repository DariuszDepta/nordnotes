#!/usr/bin/env bash

# prepare JSON data file containing login and password
echo -n '{"login":"bob", "password":"bob1234"}' >> data.json

# call login endpoint
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:8871/api/v1/login

# delete data file
rm data.json

