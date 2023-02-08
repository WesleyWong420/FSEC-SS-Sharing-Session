#!/bin/bash
docker rm -f knockknock
docker build --tag=knockknock . && \
docker run -p 9002:9002 --privileged=true --restart=on-failure --name=knockknock --detach knockknock
docker exec knockknock //bin/bash -c "/challenge/restart.sh > /dev/null 2>&1"