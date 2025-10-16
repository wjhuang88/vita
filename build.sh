#!/bin/bash
BASE_PATH="$(dirname "$(readlink -f "$0")")"

IMAGE_TAG=framework-builder:$RANDOM
docker build "$BASE_PATH" -f "$BASE_PATH/Dockerfile.build" -t $IMAGE_TAG
exec docker run --rm -v "$BASE_PATH":/apps $IMAGE_TAG sh -c "/apps/gradlew clean build"