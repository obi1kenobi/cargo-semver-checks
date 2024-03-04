#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel)"
LINTS_DIR="$TOPLEVEL/src/lints"
TEST_CRATES_DIR="$TOPLEVEL/test_crates"
TEST_OUTPUTS_DIR="$TOPLEVEL/test_outputs"
SRC_QUERY_FILE="$TOPLEVEL/src/query.rs"

# Make the script cwd-independent by always moving to the repo root first.
cd "$TOPLEVEL"

# What should the lint be called?
set +u
NEW_LINT_NAME="$1"
if [[ "$NEW_LINT_NAME" == "" || "$NEW_LINT_NAME" == "--help" ]]; then
    echo "Specify the name of the lint to add: make_new_lint.sh <LINT_NAME>"
    exit 1
fi
set -u

# Make the lint file.
LINT_FILENAME="$LINTS_DIR/$NEW_LINT_NAME.ron"
echo -n "Creating lint definition file ${LINT_FILENAME#"$TOPLEVEL/"} ..."
if [[ -f "$LINT_FILENAME" ]]; then
    echo ' already exists.'
else
    cat <<EOF >"$LINT_FILENAME"
SemverQuery(
    id: "$NEW_LINT_NAME",
    human_readable_name: "TODO",
    description: "TODO",
    required_update: Major,  // TODO
    reference_link: None,  // TODO
    query: r#"
    {
        CrateDiff {
            # TODO
        }
    }"#,
    arguments: {
        // TODO
    },
    error_message: "TODO",
    per_result_error_template: Some("TODO"),
)
EOF
    echo ' done!'
fi

# Make the test crates.
NEW_LINT_TEST_CRATES_DIR="$TEST_CRATES_DIR/$NEW_LINT_NAME"
./scripts/make_new_test_crate.sh "$NEW_LINT_NAME"

# Add the test outputs file.
NEW_TEST_OUTPUT_FILE="$TEST_OUTPUTS_DIR/$NEW_LINT_NAME.output.ron"
echo -n "Creating test outputs file ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"} ..."
if [[ -f "$NEW_TEST_OUTPUT_FILE" ]]; then
    echo ' already exists.'
else
    cat <<EOF >"$NEW_TEST_OUTPUT_FILE"
{
    "./test_crates/$NEW_LINT_NAME/": [
        // TODO
    ]
}
EOF
    echo ' done!'
fi

# Add the new lint to the `add_lints!()` macro.
echo -n "Registering lint in src/query.rs ..."
if awk -v lint_name="$NEW_LINT_NAME" '
    /^add_lints!\(/ { searching = 1 }
    searching && $0 ~ "[[:space:]]" lint_name "," { found = 1; exit }
    END { if (found) { exit 0 } else { exit 1 } }
' "$SRC_QUERY_FILE"; then
    printf ' already exists.\n'
else
    tmp=${SRC_QUERY_FILE}.tmp
    sed -e '/^add_lints!($/ a\'"
    $NEW_LINT_NAME," "$SRC_QUERY_FILE" > "$tmp" && mv -- "$tmp" "$SRC_QUERY_FILE" || {
        code=$?
        rm -f "$tmp"
        exit "$code"
    }
    printf ' done!\n'
fi

echo ''
echo 'Lint created successfully! Remember to:'
echo "- implement the lint in ${LINT_FILENAME#"$TOPLEVEL/"}"
echo "- populate the test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"}"
echo "- add the expected test outputs in ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"}"
