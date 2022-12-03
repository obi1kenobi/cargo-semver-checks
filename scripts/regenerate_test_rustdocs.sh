#!/usr/bin/env bash

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

export CARGO_TARGET_DIR=/tmp/test_crates
RUSTDOC_OUTPUT_DIR="$CARGO_TARGET_DIR/doc"
TOPLEVEL="$(git rev-parse --show-toplevel)"
TARGET_DIR="$TOPLEVEL/localdata/test_data"

# Allow setting an explicit toolchain, like +nightly or +beta.
set +u
TOOLCHAIN="$1"
set -u
RUSTDOC_CMD="cargo $TOOLCHAIN rustdoc"

# Run rustdoc on test_crates/*/{new,old}/
for crate_pair in $(ls "$TOPLEVEL/test_crates"); do
    for crate_version in "new" "old"; do
        crate="$crate_pair/$crate_version"
        echo "Generating: $crate"

        pushd "$TOPLEVEL/test_crates/$crate"
        RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD -- -Zunstable-options --output-format json
        mkdir -p "$TARGET_DIR/$crate"
        mv "$RUSTDOC_OUTPUT_DIR/$crate_pair.json" "$TARGET_DIR/$crate/rustdoc.json"
        popd
    done
done

unset CARGO_TARGET_DIR
