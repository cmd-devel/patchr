#! /usr/bin/env bash

source "$(dirname $0)/../prolog.sh"

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

# One should not be able to register the same repo multiple times
# even if the register command is not executed from the same directory.
# However, it should be possible to deal with submodules.
test_register_subdir() {
    r1="$(create_test_repo)"
    r2="$(create_test_repo)"
    cd "$r1"

    mkdir test
    cd test
    echo content > file
    git add file
    git commit -m 'A file'
    cd ..
    git -c protocol.file.allow=always submodule add "$r2" r2

    cd test
    run register r
    cd ..
    (! run register rparent)
    cd r2
    run register r2
    cd ..
    repo_has_dir r "$r1"
    repo_has_dir r2 "$r1/r2"

    run create foo 'A series'
    run list | grep -q ' foo (v1)'
    cd test
    run list | grep -q ' foo (v1)'
    cd ../r2
    (! run list | grep -q ' foo (v1)')
    cd ..

    cd r2
    run create foo2 'Another series'
    run list | grep -q ' foo2 (v1)'
    cd ..
    run list | grep -q ' foo (v1)'
    (! run list | grep -q ' foo2 (v1)')

    cd test
    run delrepo
    ! known_repo r
    known_repo r2
    cd ..
    ! known_repo r
    known_repo r2
    cd r2
    run delrepo
    ! known_repo r
    ! known_repo r2
    cd ..
    ! known_repo r
    ! known_repo r2
}

run_test_funcs test_register_delete test_list test_register_subdir
