#!/bin/bash

cargo build --release --target=x86_64-unknown-linux-gnu --target-dir target-22.04
chown -R 1000:1000 target-22.04
