#!/bin/sh
DOCKER_CONTAINER=dbday
DOCKER_IMAGE=dbday
DOCKER_TAG=latest
DOCKER_FILE=$(cat $HOME/.tmp-docker-deployfile)
DOCKER_START_SCRIPT="$HOME/.scripts/dbday_run.sh"
rm $HOME/.tmp-docker-deployfile
docker stop ${DOCKER_CONTAINER} && docker rm ${DOCKER_CONTAINER}
docker rmi ${DOCKER_IMAGE}:${DOCKER_TAG}
docker load < ${DOCKER_FILE}
rm -rf ${DOCKER_FILE}
sh ${DOCKER_START_SCRIPT}