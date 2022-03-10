#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
GOPATH=$CURRENT_DIR
GOOS=linux

export GOPATH
export GOOS

go install tests

