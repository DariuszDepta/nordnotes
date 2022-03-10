#!/usr/bin/env bash

gunicorn -b 0.0.0.0:8871 nordnotes:app --reload