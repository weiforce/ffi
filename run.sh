#!/bin/bash
ACCESS="--privileged --net=host"
#GPU="--gpus all"
X11="$GPU -e DISPLAY=:0 -v /tmp/.X11-unix:/tmp/.X11-unix"
PROXY="-e HTTP_PROXY=http://127.0.0.1:7890 -e HTTPS_PROXY=http://127.0.0.1:7890"
ENTRYPOINT="--entrypoint="
VOLUME="-v /home/core:/home/core"
PARAM="$ACCESS $VOLUME $X11 $VIDEO $PROXY $ENTRYPOINT"
COMMAND="sh"
REPO="ffi"

xhost +
sudo docker run -it --rm $PARAM weiforce/$REPO $COMMAND
