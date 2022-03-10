#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
GOPATH=$CURRENT_DIR

export GOPATH

echo "Removing existing dependencies..."

rm -rf "$CURRENT_DIR/src/github.com"

echo "Downloading newest dependencies..."

echo "Getting: uuid"
go get github.com/google/uuid

echo "Getting: jose"
go get github.com/SermoDigital/jose

echo "Getting: oxyde"
go get github.com/EngosSoftware/oxyde

echo "Downloading dependencies completed."