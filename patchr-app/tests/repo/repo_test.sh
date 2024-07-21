#! /usr/bin/env bash

source "$(dirname $0)/../prolog.sh"

create_test_repo() {
    d=$(mktemp -d -p "$TMP_DIR")
    (cd "$d" && git init .) > /dev/null 2>&1
    echo "$d"
}

test_list() {
    r1=$(create_test_repo)
    r2=$(create_test_repo)

    out=$("$BIN" repos; exit $?)
    test -z $out

    cd "$r1"
    out=$(run repos; exit $?)
    test -z $out

    # TODO
    exit 1 # FAIL
}

test_list