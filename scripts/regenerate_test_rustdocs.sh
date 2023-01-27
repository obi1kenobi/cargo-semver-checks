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
echo "Generating rustdoc with: $(cargo $TOOLCHAIN --version)"
RUSTDOC_CMD="cargo $TOOLCHAIN rustdoc"

# Run rustdoc on test_crates/*/{new,old}/
for crate_pair in $(find "$TOPLEVEL/test_crates/" -maxdepth 1 -mindepth 1 -type d); do
    # Removing path prefix, leaving only the directory name without forward slashes
    crate_pair=${crate_pair#"$TOPLEVEL/test_crates/"}

    if [[ -f "$TOPLEVEL/test_crates/$crate_pair/new/Cargo.toml" ]]; then
        if [[ -f "$TOPLEVEL/test_crates/$crate_pair/old/Cargo.toml" ]]; then
            for crate_version in "new" "old"; do
                crate="$crate_pair/$crate_version"
                echo "Generating: $crate"

                pushd "$TOPLEVEL/test_crates/$crate"
                RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD -- -Zunstable-options --document-private-items --document-hidden-items --output-format=json
                mkdir -p "$TARGET_DIR/$crate"
                mv "$RUSTDOC_OUTPUT_DIR/$crate_pair.json" "$TARGET_DIR/$crate/rustdoc.json"
                popd
            done
        else
            echo >&2 "WARNING: $crate_pair/new/Cargo.toml exists but $crate_pair/old/Cargo.toml does not; skipping $crate_pair."
        fi
    fi
done

unset CARGO_TARGET_DIR
