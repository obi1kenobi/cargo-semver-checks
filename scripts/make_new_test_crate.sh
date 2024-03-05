#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel)"
TEST_CRATES_DIR="$TOPLEVEL/test_crates"

# Make the script cwd-independent by always moving to the repo root first.
cd "$TOPLEVEL"

# What should the new test crate be called?
set +u
NEW_TEST_CRATE="$1"
if [[ "$NEW_TEST_CRATE" == "" || "$NEW_TEST_CRATE" == "--help" ]]; then
    echo "Specify the name of the test crate to add: make_new_test_crate.sh <CRATE_NAME>"
    exit 1
fi
set -u

NEW_LINT_TEST_CRATES_DIR="$TEST_CRATES_DIR/$NEW_TEST_CRATE"
echo -n "Creating test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"} ..."
if [[ -d "$NEW_LINT_TEST_CRATES_DIR" ]]; then
    echo ' already exists.'
else
    cp -R "$TEST_CRATES_DIR/template" "$NEW_LINT_TEST_CRATES_DIR"
    sed -e "s/template/$NEW_TEST_CRATE/g" "$TEST_CRATES_DIR/template/old/Cargo.toml" > "$NEW_LINT_TEST_CRATES_DIR/old/Cargo.toml"
    sed -e "s/template/$NEW_TEST_CRATE/g" "$TEST_CRATES_DIR/template/new/Cargo.toml" > "$NEW_LINT_TEST_CRATES_DIR/new/Cargo.toml"
    echo ' done!'
fi
