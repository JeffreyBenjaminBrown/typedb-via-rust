# This is not a script, just snippets.
# They are based around the image built by the Dockerfile at
#   git@github.com:JeffreyBenjaminBrown/docker-typedb-rust.git
#   /home/jeff/hodal/docker-typedb-rust

exit # Because this is not a script.

CONTAINER_NAME=rust-typedb

docker exec -it $CONTAINER_NAME bash

docker stop $CONTAINER_NAME && docker rm $CONTAINER_NAME

STARTING_AT=$(date)
echo $(date)
docker build -t jeffreybbrown/hode:new .
echo $(date)

DOCKER_IMAGE_SUFFIX="2025-03-13.rust-no-python"
docker tag jeffreybbrown/hode:new jeffreybbrown/hode:latest
docker tag jeffreybbrown/hode:new jeffreybbrown/hode:$DOCKER_IMAGE_SUFFIX
docker rmi jeffreybbrown/hode:new

docker run --name $CONTAINER_NAME -it -d        \
 -v /home/jeff/hodal/typedb-via-rust:/home/user \
 -p 1729:1729                                   \
 --platform linux/amd64                         \
 jeffreybbrown/hode:latest

docker push jeffreybbrown/hode:$DOCKER_IMAGE_SUFFIX
docker push jeffreybbrown/hode:latest
