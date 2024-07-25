DATA_DIR="$(realpath $HOME/.patchr)"
ROOT_FILE="$DATA_DIR/root.json"

run() {
    "$BIN" "$@"
}

repo_file() {
    local repo_name="$1"
    echo "$DATA_DIR/$repo_name"
}

clear_data_dir() {
    # Make sure we are not doing bad things
    if ! echo "$HOME" | grep -q '^/tmp' || [ -z "$DATA_DIR" ]
    then
        exit 1
    fi

    # And then delete the data dir
    rm -rf "$DATA_DIR"
    # Run without argument to re-init the data dir
    run
}

run_test_funcs() {
    tests=("$@")
    for t in "${tests[@]}"
    do
        clear_data_dir
        "$t"
    done
}

check_json() {
    local file="$1"
    local filter="$2"
    local expected_output="$3"
    out=$(jq "$filter" < "$file")
    echo "$out" | grep -q "$expected_output"
}

# Convenient wrapper
check_json_root_file() {
    check_json $ROOT_FILE "${@:1}"
}

# This function does not fail
# It's up to the caller to fail if necessary
known_repo() {
    local name="$1"
    if [ ! -f "$DATA_DIR/$name" ]
    then
        return 1
    fi

    if ! check_json_root_file "[.repos[] | select(.name | . == \"$name\")] | length" 1
    then
        return 1
    fi

    return 0
}