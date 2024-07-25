#! /usr/bin/env bash

source "$(dirname $0)/../prolog.sh"

create_test_repo() {
    d=$(mktemp -d -p "$TMP_DIR")
    (cd "$d" && git init .) > /dev/null 2>&1
    echo "$d"
}

test_register_delete() {
    check_json_root_file '.repos | length' 0

    # Register should fail when not in a repo
    ! run register test
    ! known_repo test

    r="$(create_test_repo)"
    cd "$r"

    ! known_repo test
    run register test
    known_repo test
    cd -
    known_repo test

    cd "$r"
    run delrepo
    ! known_repo test
}

test_list() {
    r1="$(create_test_repo)"
    r2="$(create_test_repo)"
    cur=$(pwd)

    # Not in a repo
    out=$(run repos)
    [ -z $out ]

    # From an unregistered repo
    cd "$r1"
    out=$(run repos)
    [ -z $out ]

    # Register a repo
    run register repo1

    cd "$cur"

    # Not in a repo
    out=$(run repos)
    [ $(echo "$out" | wc -l) = 1 ]
    [ $(echo "$out" | grep "^- repo1" | wc -l) = 1 ]

    cd "$r1"
    out=$(run repos)
    [ $(echo "$out" | wc -l) = 1 ]
    [ $(echo "$out" | grep "^- repo1" | wc -l) = 1 ]

    # From an unregistered repo
    cd "$r2"
    out=$(run repos)
    [ $(echo "$out" | wc -l) = 1 ]
    [ $(echo "$out" | grep "^- repo1" | wc -l) = 1 ]
}

run_test_funcs test_register_delete test_list
