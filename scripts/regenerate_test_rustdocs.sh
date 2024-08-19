#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

dir_is_newer_than_file() {
    local dir="$1"
    local file="$2"
    [[ ! -e $file ]] || [[ $(find "$dir" -newer "$file" -exec sh -c 'printf found; kill "$PPID"' \;) ]]
}

export CARGO_TARGET_DIR=/tmp/test_crates
RUSTDOC_OUTPUT_DIR="$CARGO_TARGET_DIR/doc"

# Get the top-level directory of the project (the repo):
# - If the user has cloned the git repo, ask git.
# - Otherwise, navigate up relative to this script's path.
#   This latter option is useful when building cargo-semver-checks for distribution with NixOS:
#   https://github.com/obi1kenobi/cargo-semver-checks/issues/855
TOPLEVEL="$(git rev-parse --show-toplevel 2>/dev/null || { cd -- "$(dirname -- "${BASH_SOURCE[0]}" )" &>/dev/null && cd -- .. &>/dev/null && pwd; })"
TARGET_DIR="$TOPLEVEL/localdata/test_data"
TOOLCHAIN=

# Allow setting an explicit toolchain, like +nightly or +beta.
set +u
if [[ $1 == +* ]]; then
    TOOLCHAIN="$1"
    shift
fi
set -u
echo "Generating rustdoc with: $(cargo $TOOLCHAIN --version)"
RUSTDOC_CMD="cargo $TOOLCHAIN rustdoc"

# Run rustdoc on test_crates/*/{new,old}/
if [[ $# -eq 0 ]]; then
    set -- "$TOPLEVEL/test_crates/"*/
    always_update=
else
    always_update=1
fi
for crate_pair; do
    # Strip all but last path component from crate_pair
    crate_pair=${crate_pair%/}
    crate_pair=${crate_pair##*/}

    if [[ -f "$TOPLEVEL/test_crates/$crate_pair/new/Cargo.toml" ]]; then
        if [[ -f "$TOPLEVEL/test_crates/$crate_pair/old/Cargo.toml" ]]; then
            for crate_version in "new" "old"; do
                crate="$crate_pair/$crate_version"
                crate_dir=$TOPLEVEL/test_crates/$crate
                target=$TARGET_DIR/$crate/rustdoc.json

                if [[ -z $always_update ]] && ! dir_is_newer_than_file "$crate_dir" "$target"; then
                    printf 'No updates needed for %s.\n' "$crate"
                    continue
                fi

                echo "Generating: $crate"

                pushd "$crate_dir"

                # Determine whether to warn on lints or allow them.
                CAP_LINTS="warn"
                if [[ "$crate" == "broken_rustdoc" ]]; then
                    # This crate *intentionally* has broken rustdoc.
                    # Don't warn on it. The warnings and errors are thing being tested.
                    CAP_LINTS="allow"
                fi

                RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD -- -Zunstable-options --document-private-items --document-hidden-items --cap-lints "$CAP_LINTS" --output-format=json
                mkdir -p "$TARGET_DIR/$crate"
                mv "$RUSTDOC_OUTPUT_DIR/$crate_pair.json" "$target"
                popd
            done
        else
            echo >&2 "WARNING: $crate_pair/new/Cargo.toml exists but $crate_pair/old/Cargo.toml does not; skipping $crate_pair."
        fi
    fi
done

unset CARGO_TARGET_DIR
