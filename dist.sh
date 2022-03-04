#!/bin/sh
PATH=/usr/bin

docker build -t registry.box/dbday:latest .
docker push registry.box/dbday:latest