#!/usr/bin/bash

project=${1:?Usage: runner.sh tasks/NAME/src/main.rs [program arguments...]}
project=${project#*/}
project=${project%%/*}
shift

echo "Running $project"

cargo build --release --bin "$project" || exit $?

(
    # These limits are specified in KiB.
    ulimit -v $((5 * 1024 * 1024))
    ulimit -s $((512 * 1024))
    RUST_BACKTRACE=1 exec "./target/release/$project" "$@"
)
