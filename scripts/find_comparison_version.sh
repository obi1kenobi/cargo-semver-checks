#!/usr/bin/env bash

# Script requirements:
# - curl
# - jq
# - sort with `-V` flag, available in `coreutils-7`
#   On macOS this may require `brew install coreutils`.

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go to the repo root directory.
cd "$(git rev-parse --show-toplevel)"

# The first argument should be the name of a crate.
CRATE_NAME="$1"

CURRENT_VERSION="$( \
    cargo metadata --format-version 1 | \
    jq --arg crate_name "$CRATE_NAME" --exit-status -r \
        '.packages[] | select(.name == $crate_name) | .version' \
)" || (echo >&2 "No crate named $CRATE_NAME found in workspace."; exit 1)
echo >&2 "Crate $CRATE_NAME current version: $CURRENT_VERSION"

# The leading whitespace is important! With it, we know that every version is both
# preceded by and followed by whitespace. We use this fact to avoid matching
# on substrings of versions.
EXISTING_VERSIONS="
$( \
    curl 2>/dev/null "https://crates.io/api/v1/crates/$CRATE_NAME" | \
    jq --exit-status -r .versions[].num \
)"
echo >&2 -e "Versions on crates.io:$EXISTING_VERSIONS\n"

# Use version sort (sort -V) to get all versions in ascending order, then use grep to:
# - grab the first line that matches the current version (--max-count=1)
# - only match full lines (--line-regexp)
# - get one line of leading context (-B 1) i.e. the immediately-smaller version, if one exists
# - explicitly opt out of trailing context lines (-A 0)
# Finally, use `head` to output only the first of the up-to-two lines output.
# Now, either:
# - two lines were output, and we grabbed the immediately-smaller version, or
# - one line was output with only our version, because there was no immediately-smaller version,
#   and we grabbed that one. We sort this out with the subsequent conditional.
OUTPUT="$( \
    echo -e "$CURRENT_VERSION$EXISTING_VERSIONS" | \
    sort -V | \
    grep -B 1 -A 0 --line-regexp --max-count=1 "$CURRENT_VERSION" | \
    head -n 1 \
)"

if [[ "$OUTPUT" == "$CURRENT_VERSION" ]]; then
    echo >&2 "There is no suitable comparison version."
    echo >&2 \
        "The current version $CURRENT_VERSION is smaller than any version published on crates.io"
    exit 1
fi

echo "Comparison version: $OUTPUT" >&2
echo "$OUTPUT"
