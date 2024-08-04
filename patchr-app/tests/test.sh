#! /usr/bin/env bash

cd $(dirname $0)

BIN="$(realpath ../target/release/patchr)"

export FAKE_EDITOR="$(pwd)/fake_editor.sh"

source ./util.sh

set -ex

init_tmp_env() {
    local base_dir="$1"
    mktemp -d -p "$base_dir"
}

setup_git() {
    git config --global user.name 'Patchr'
    git config --global user.email 'test@patchr.com'
}

clean_tmp_env() {
    local tmp_dir="$1"
    rm -rf "$tmp_dir"
}

get_all_tests() {
    find . -mindepth 2 -type f
}

run_tests() {
    local tmp_dir="$1"
    local tests_to_run=("${@:2}")
    for t in "${tests_to_run[@]}"
    do
        echo "Running $t... "
        clear_data_dir
        "$t" "$BIN" "$tmp_dir"
    done

    echo "All tests passed"
}

# Run cargo test first
cargo build -r
cargo test

tmp_dir="$(init_tmp_env /tmp)"
export HOME="$tmp_dir" # use a temporary .patchr directory
setup_git

# Check if the user wants to run all the tests or just a subset
if [ "$#" -ge 1 ]
then
    tests_to_run="$@"
else
    tests_to_run=( $(get_all_tests) )
fi

run_tests "$tmp_dir" "${tests_to_run[@]}"

clean_tmp_env "$tmp_dir"
