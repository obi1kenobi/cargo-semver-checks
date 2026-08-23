#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

set -euo pipefail

# Go to repo root
TOPLEVEL="$(git rev-parse --show-toplevel 2>/dev/null || { cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/.. && pwd; })"
cd "$TOPLEVEL"

{
    printf '%s\0' scripts/regenerate_test_rustdocs.sh
    find test_crates \
        -type f \( -name 'Cargo.toml' -o -name '*.rs' \) \
        -not -path '*/target/*' \
        -not -name 'Cargo.lock' \
        -print0
} | LC_ALL=C sort -z | xargs -0 sha256sum --zero -- | while IFS= read -r -d '' record; do
    # `sha256sum --zero` emits: 64 hex digits, a space, a mode character,
    # the filename, and a NUL terminator.
    file_hash="${record:0:64}"
    file="${record:66}"
    printf '%s\0%s\0' "${file#./}" "$file_hash"
done | sha256sum | cut -d' ' -f1
