#!/bin/sh
IP_SERVER=192.168.178.100
SSH_USER=aaron
FILE_NAME="/.tmp-$(head -c 4 /dev/urandom | base32 | head -c 6)"
docker build -t dbday:latest .
cd ./package
docker save dbday:latest > dbday.docker.tar
scp dbday.docker.tar aaron@gitea.git:~${FILE_NAME}
cd ..
ssh ${SSH_USER}@${IP_SERVER} "echo \"\$HOME${FILE_NAME}\" > ~/.tmp-docker-deployfile; bash -s" < deploy/deploy_remote.sh