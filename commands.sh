exit # This is not a script, just snippets.

docker exec -it rust-typedb bash

docker stop rust-typedb && docker rm rust-typedb

STARTING_AT=$(date)
echo $(date)
docker build -t jeffreybbrown/hode:new .
echo $(date)

docker run --name rust-typedb -it -d           \
 -v /home/jeff/code/rust-for-typedb:/home/user \
 -p 1729:1729                                  \
 --platform linux/amd64                        \
 jeffreybbrown/hode:latest

DOCKER_IMAGE_SUFFIX="2025-03-13.rust-no-python"
docker tag jeffreybbrown/hode:new jeffreybbrown/hode:latest
docker tag jeffreybbrown/hode:new jeffreybbrown/hode:$DOCKER_IMAGE_SUFFIX
docker rmi jeffreybbrown/hode:new

docker push jeffreybbrown/hode:$DOCKER_IMAGE_SUFFIX
docker push jeffreybbrown/hode:latest
