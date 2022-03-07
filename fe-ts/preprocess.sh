#!/bin/bash

# clear distribution directory
rm -rf dist/*

# retrieve the version and hash number from package.json
version=$(grep -E '\"version\".*:.*[^,]' package.json | tr -d \", | awk '{print $2}')
echo "NordNotes version: $version"
number=${version//\.}
echo "NordNotes hash number: $number"

# update output name in webpack.common.js
sed -i "s/'nordnotes.*\.js'/\'nordnotes-$number\.js'/" webpack.common.js
echo "NordNotes updated: webpack.common.js"

# update bundle name in src/index.html
sed -i "s/\"nordnotes.*\.js\"/\"nordnotes-$number\.js\"/" ./src/index.html
echo "NordNotes updated: src/index.html"
