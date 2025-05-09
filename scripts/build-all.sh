#!/bin/bash

pushd "$(dirname "$(realpath "$0")")/.." || {
    echo "Failed to change directory"; exit 1;
}

# docker build -f Dockerfile-22.04 -t rust-build:22.04 .
# docker build -f Dockerfile-24.04 -t rust-build:24.04 .

docker run -v .:/src rust-build:22.04 bash ./scripts/build-22.04-docker.sh
docker run -v .:/src rust-build:24.04 bash ./scripts/build-24.04-docker.sh

# cargo build --release --target-dir target-24.04 --target=x86_64-unknown-linux-gnu

popd
