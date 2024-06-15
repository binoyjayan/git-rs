#!/bin/bash

script_path="$(dirname "$(readlink -f "$0")")"

mkdir -p /tmp/git_dir
cd /tmp/git_dir

cargo run \
    --quiet \
    --target-dir="$script_path/target" \
    --manifest-path "$script_path/Cargo.toml" "$@"
