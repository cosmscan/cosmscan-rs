#!/bin/sh

docker stop $(docker ps -q)

docker container prune
docker image prune -a
docker volume prune