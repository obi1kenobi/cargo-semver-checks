#!/usr/bin/env bash

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go to the semver_tests directory.
cd "$(git rev-parse --show-toplevel)/semver_tests"

export CARGO_TARGET_DIR=/tmp/semver_tests
RUSTDOC_OUTPUT="$CARGO_TARGET_DIR/doc/semver_tests.json"
TARGET_DIR="$(git rev-parse --show-toplevel)/localdata/test_data"

# Ensure the target test data directory exists.
mkdir -p "$TARGET_DIR"

# Make the baseline configuration file.
echo "Generating: baseline"
cargo +nightly-2022-08-30 rustdoc -- -Zunstable-options --output-format json
mv "$RUSTDOC_OUTPUT" "$TARGET_DIR/baseline.json"

# For each feature, re-run rustdoc with it enabled.
features="$(cargo metadata --format-version 1 | \
    jq --exit-status -r '.packages[] | select(.name = "semver_tests") | .features | keys[]')"
while IFS= read -r feat; do
    echo "Generating: $feat"
    cargo +nightly-2022-08-30 rustdoc --features "$feat" -- -Zunstable-options --output-format json
    mv "$RUSTDOC_OUTPUT" "$TARGET_DIR/$feat.json"
done <<< "$features"

unset CARGO_TARGET_DIR
