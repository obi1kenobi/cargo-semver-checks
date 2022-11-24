#!/usr/bin/env bash

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go to the test_crates directory.
cd "$(git rev-parse --show-toplevel)/test_crates"

export CARGO_TARGET_DIR=/tmp/test_crates
RUSTDOC_OUTPUT="$CARGO_TARGET_DIR/doc/test_crates.json"
TARGET_DIR="$(git rev-parse --show-toplevel)/localdata/test_data"

# Allow setting an explicit toolchain, like +nightly or +beta.
set +u
TOOLCHAIN="$1"
set -u
RUSTDOC_CMD="cargo $TOOLCHAIN rustdoc"

# Ensure the target test data directory exists.
mkdir -p "$TARGET_DIR"

# Make the baseline configuration file.
echo "Generating: baseline"
RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD -- -Zunstable-options --output-format json
mv "$RUSTDOC_OUTPUT" "$TARGET_DIR/baseline.json"

# For each feature, re-run rustdoc with it enabled.
features="$(cargo metadata --format-version 1 | \
    jq --exit-status -r '.packages[] | select(.name = "test_crates") | .features | keys[]')"
while IFS= read -r feat; do
    echo "Generating: $feat"
    RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD --features "$feat" -- -Zunstable-options --output-format json
    mv "$RUSTDOC_OUTPUT" "$TARGET_DIR/$feat.json"
done <<< "$features"

unset CARGO_TARGET_DIR
