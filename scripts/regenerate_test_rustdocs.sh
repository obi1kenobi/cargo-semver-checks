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

CARGO_TARGET_DIR_BASE=/tmp/test_crates
export CARGO_TARGET_DIR="$CARGO_TARGET_DIR_BASE"

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
METADATA_CMD="cargo $TOOLCHAIN metadata --format-version 1"

if [[ $# -eq 0 ]]; then
    # Run rustdoc on test_crates/*/{new,old}/
    set -- "$TOPLEVEL/test_crates/"*/
    always_update=
else
    # Run on whichever paths the user specified.
    if [[ $1 == '*' ]]; then
        # As a special case, run on everything if the user specified a literal (escaped) asterisk.
        set -- "$TOPLEVEL/test_crates/"*/
    fi
    always_update=1
fi

# Determine parallelism. Respect $NUM_JOBS if provided.
NUM_JOBS=${NUM_JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 1)}

PLACEHOLDER_DIR_BASE="$TARGET_DIR/placeholder"
rm -rf "${PLACEHOLDER_DIR_BASE}"*
mkdir -p "$TARGET_DIR"
pushd "$TARGET_DIR" >/dev/null
for i in $(seq 0 $((NUM_JOBS - 1))); do
    cargo new --lib "placeholder${i}"
    pushd "placeholder${i}" >/dev/null
    cargo add --path ../../../test_crates/template/old/
    popd >/dev/null
done
popd >/dev/null

generate_rustdocs() {
    local crate="$1"
    local crate_dir="$TOPLEVEL/test_crates/$crate"
    local pair="${crate%%/*}"
    local target="$TARGET_DIR/$crate/rustdoc.json"
    local output_dir="$CARGO_TARGET_DIR/doc"

    if [[ -z $always_update ]] && ! dir_is_newer_than_file "$crate_dir" "$target"; then
        printf 'No updates needed for %s.\n' "$crate"
        return
    fi

    echo "Generating: $crate"

    pushd "$crate_dir" >/dev/null

    CAP_LINTS="warn"
    if [[ "$crate" == "broken_rustdoc" ]]; then
        CAP_LINTS="allow"
    fi

    RUSTC_BOOTSTRAP=1 $RUSTDOC_CMD -- -Zunstable-options --document-private-items --document-hidden-items --cap-lints "$CAP_LINTS" --output-format=json
    mkdir -p "$TARGET_DIR/$crate"
    mv "$output_dir/$pair.json" "$target"
    popd >/dev/null
}

generate_metadata() {
    local crate="$1"
    local metadata="$TARGET_DIR/$crate/metadata.json"

    pushd "$PLACEHOLDER_DIR" >/dev/null
    sed -i='' '$d' Cargo.toml
    cargo add --path "../../../test_crates/$crate"
    $METADATA_CMD >"$metadata"
    popd >/dev/null
}
export TOPLEVEL TARGET_DIR RUSTDOC_CMD METADATA_CMD PLACEHOLDER_DIR_BASE always_update
export -f generate_rustdocs generate_metadata dir_is_newer_than_file

crate_jobs=()
for crate_pair; do
    crate_pair=${crate_pair%/}
    crate_pair=${crate_pair##*/}

    if [[ -f "$TOPLEVEL/test_crates/$crate_pair/new/Cargo.toml" ]]; then
        if [[ -f "$TOPLEVEL/test_crates/$crate_pair/old/Cargo.toml" ]]; then
            crate_jobs+=("$crate_pair/new")
            crate_jobs+=("$crate_pair/old")
        else
            echo >&2 "WARNING: $crate_pair/new/Cargo.toml exists but $crate_pair/old/Cargo.toml does not; skipping $crate_pair."
        fi
    fi
done

for i in $(seq 0 $((NUM_JOBS - 1))); do
    (
        export CARGO_TARGET_DIR="${CARGO_TARGET_DIR_BASE}/worker${i}"
        for ((j=i; j<${#crate_jobs[@]}; j+=NUM_JOBS)); do
            generate_rustdocs "${crate_jobs[j]}"
        done
    ) &
done
wait
for i in $(seq 0 $((NUM_JOBS - 1))); do
    (
        export PLACEHOLDER_DIR="${PLACEHOLDER_DIR_BASE}${i}"
        for ((j=i; j<${#crate_jobs[@]}; j+=NUM_JOBS)); do
            generate_metadata "${crate_jobs[j]}"
        done
    ) &
done
wait

unset CARGO_TARGET_DIR PLACEHOLDER_DIR
