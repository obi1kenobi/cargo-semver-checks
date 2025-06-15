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

find test_crates \
    -type f \( -name 'Cargo.toml' -o -name '*.rs' \) \
    -not -path '*/target/*' \
    -not -name 'Cargo.lock' \
    -print0 | sort -z | while IFS= read -r -d '' file; do
    printf '%s\n' "${file#./}"
    cat "$file"
done | sha256sum | cut -d' ' -f1

