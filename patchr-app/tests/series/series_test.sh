#! /usr/bin/env bash

source "$(dirname $0)/../prolog.sh"

test_create_series() {
    check_json_root_file '.repos | length' 0

    r="$(create_test_repo)"
    cd "$r"

    run register r
    check_json_root_file '.repos | length' 1
    run create s1 'Test series'
    expected_short_name=$(basename "$r") # Check that the short name is infered from the repo dir name
    run show -v s1 | grep -q "^Short name : $expected_short_name$"

    cd ..
    new_r="$r+" # Invalid, should not be used as a short name
    mv "$r" "$new_r"
    cd "$new_r"
    run register new_r
    run create s1 'Test series' | grep -q "The repo name cannot be used as a short name"
    run show -v s1 | grep -q "^Short name : $"
}

test_cv_skel() {
    r="$(create_test_repo)"
    cd "$r"

    # No skel
    run register r
    run create s1 'Test series'
    out=$(run show s1)
    [ -z "$out" ] # Empty CV by default

    # Define a cv skel
    skel='This is a test cv skel'
    setup_fake_editor "$skel"
    run cvskel
    run create s2 'Test series'
    out=$(run show s2)
    [ "$out" = "$skel" ]
}

# Set and clear the short name
test_edit_short_name() {
    r=$(create_test_repo)
    cd "$r"
    run register r
    run create s 'Test series'
    setup_fake_editor "shname"
    run edit short s
    run show -v s | grep -q '^Short name : shname$'
    setup_fake_editor ''
    run edit short s
    run show -v s | grep -q '^Short name : $'
}

run_test_funcs test_create_series test_cv_skel test_edit_short_name