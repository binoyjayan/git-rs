#!/bin/bash

script_path="$(dirname "$(readlink -f "$0")")"

cd git_dir

cargo run \
    --quiet \
    --target-dir=../target \
    --manifest-path "$script_path/Cargo.toml" "$@"
