#!/usr/bin/bash

project=$(echo "$1" | tr '/' ' ' | awk '{print $2 }')
echo "Running $project"
RUST_BACKTRACE=1 cargo run --bin "$project" "${@:2}"