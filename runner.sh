#!/usr/bin/bash

project=$(echo "$1" | tr '/' ' ' | awk '{print $2 }')
echo "Running $project"

cargo build --bin "$project" || exit $?

(
    # These limits are specified in KiB.
    ulimit -v $((2 * 1024 * 1024))
    ulimit -s $((512 * 1024))
    RUST_BACKTRACE=1 exec "./target/debug/$project" "${@:2}"
)
