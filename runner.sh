#!/usr/bin/bash

project=$(echo "$1" | tr '/' ' ' | awk '{print $1 }')
RUST_BACKTRACE=1 cargo run --bin "$project" "${@:2}"