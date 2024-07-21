#! /usr/bin/env bash

PATCHR_BIN=$(realpath ../target/debug/patchr)

set -ex

init_tmp_env() {
    local base_dir="$1"
    mktemp -d -p "$base_dir"
}

clean_tmp_env() {
    local tmp_dir="$1"
    rm -rf "$tmp_dir"
}

get_all_tests() {
    find . -mindepth 2 -type f
}

run_tests() {
    local bin="$1"
    local tmp_dir="$2"
    local tests_to_run=("${@:3}")
    for t in "${tests_to_run[@]}"
    do
        echo "Running $t... "
        "$t" "$bin" "$tmp_dir"
    done

    echo "All tests passed"
}

cd "$(dirname $0)"
tmp_dir="$(init_tmp_env /tmp)"

# Check if the user wants to run all the tests or just a subset
if [ "$#" -ge 1 ]
then
    tests_to_run="$@"
else
    tests_to_run=( $(get_all_tests) )
fi

run_tests "$PATCHR_BIN" "$tmp_dir" "${tests_to_run[@]}"

clean_tmp_env "$tmp_dir"