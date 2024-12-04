#!/bin/bash

# docker build -f Dockerfile-22.04 -t rust-build:22.04 .
# docker build -f Dockerfile-24.04 -t rust-build:24.04 .

docker run -it -v .:/src rust-build:22.04 bash ./build-22.04-docker.sh
docker run -it -v .:/src rust-build:24.04 bash ./build-24.04-docker.sh

cargo build --release --target-dir target-24.04 --target=x86_64-unknown-linux-gnu
