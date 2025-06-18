#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel)"
cd "$TOPLEVEL"

OLD_NAME="${1:-}"
NEW_NAME="${2:-}"

if [[ -z "$OLD_NAME" || -z "$NEW_NAME" || "$OLD_NAME" == "--help" ]]; then
    echo "Usage: rename_lint.sh <OLD_LINT_NAME> <NEW_LINT_NAME>"
    exit 1
fi

LINTS_DIR="src/lints"
TEST_CRATES_DIR="test_crates"
TEST_OUTPUTS_DIR="test_outputs"

# Rename lint definition file and update its contents
if [[ -f "$LINTS_DIR/$OLD_NAME.ron" ]]; then
    git mv "$LINTS_DIR/$OLD_NAME.ron" "$LINTS_DIR/$NEW_NAME.ron"
    sed -i "s/id: \"$OLD_NAME\"/id: \"$NEW_NAME\"/" "$LINTS_DIR/$NEW_NAME.ron"
fi

# Rename test crates directory if it exists
if [[ -d "$TEST_CRATES_DIR/$OLD_NAME" ]]; then
    git mv "$TEST_CRATES_DIR/$OLD_NAME" "$TEST_CRATES_DIR/$NEW_NAME"
    grep -rl "$OLD_NAME" "$TEST_CRATES_DIR/$NEW_NAME" | xargs sed -i "s/$OLD_NAME/$NEW_NAME/g"
fi

# Rename snapshot files and update contents
for folder in "$TEST_OUTPUTS_DIR/query_execution" "$TEST_OUTPUTS_DIR/witnesses"; do
    for ext in snap snap.new; do
        if [[ -f "$folder/$OLD_NAME.$ext" ]]; then
            git mv "$folder/$OLD_NAME.$ext" "$folder/$NEW_NAME.$ext"
            sed -i "s/$OLD_NAME/$NEW_NAME/g" "$folder/$NEW_NAME.$ext"
        fi
    done
done

# Update lint name in src/query.rs
sed -i "s/\b$OLD_NAME\b/$NEW_NAME/" src/query.rs

# Update tests and snapshots referencing old name
grep -rl "$OLD_NAME" tests "$TEST_OUTPUTS_DIR" | xargs sed -i "s/$OLD_NAME/$NEW_NAME/g"

echo "Lint renamed from $OLD_NAME to $NEW_NAME."
