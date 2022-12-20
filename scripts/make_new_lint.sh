#!/usr/bin/env bash

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel)"
LINTS_DIR="$TOPLEVEL/src/lints"
TEST_CRATES_DIR="$TOPLEVEL/test_crates"
TEST_OUTPUTS_DIR="$TOPLEVEL/test_outputs"
SRC_QUERY_FILE="$TOPLEVEL/src/query.rs"

# What should the lint be called?
set +u
NEW_LINT_NAME="$1"
if [[ "$NEW_LINT_NAME" == "" || "$NEW_LINT_NAME" == "--help" ]]; then
    echo "Specify the name of the lint to add: make_new_lint.sh <LINT_NAME>"
    exit 1
fi
set -u

# Make the lint file. If the file already exists, bail so as not to overwrite existing lints.
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
    }
    "#,
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
echo -n "Creating test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"} ..."
if [[ -d "$NEW_LINT_TEST_CRATES_DIR" ]]; then
    echo ' already exists.'
else
    cp -R "$TEST_CRATES_DIR/template" "$NEW_LINT_TEST_CRATES_DIR"
    sed -i'' "s/template/$NEW_LINT_NAME/g" "$NEW_LINT_TEST_CRATES_DIR/old/Cargo.toml"
    sed -i'' "s/template/$NEW_LINT_NAME/g" "$NEW_LINT_TEST_CRATES_DIR/new/Cargo.toml"
    echo ' done!'
fi

# Add the test outputs file.
NEW_TEST_OUTPUT_FILE="$TEST_OUTPUTS_DIR/$NEW_LINT_NAME.output.ron"
echo -n "Creating test outputs file ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"} ..."
if [[ -f "$NEW_TEST_OUTPUT_FILE" ]]; then
    echo ' already exists.'
else
    cat <<EOF >"$NEW_TEST_OUTPUT_FILE"
[
    "./test_crates/$NEW_LINT_NAME/": [
        // TODO
    ]
]
EOF
    echo ' done!'
fi

# Add the new lint to the `add_lints!()` macro.
echo -n "Registering lint in src/query.rs ..."
set +e
# -E = extended regex mode, which behaves more similarly to regex in most programming languages.
# -z = use \0 as separators instead of \n, so that we can do multi-line matches.
grep -Ez --regexp "add_lints\!\\([^)]+[ ]+$NEW_LINT_NAME," "$SRC_QUERY_FILE" >/dev/null
OUTPUT="$?"
set -e
if [[ "$OUTPUT" == "0" ]]; then
    echo ' already exists.'
else
    # -E = extended regex mode, which behaves more similarly to regex in most programming languages.
    # -z = use \0 as separators instead of \n, so that we can do multi-line matches.
    sed -i'' -Ez "s/add_lints\!\\(([^)]+)\\)/add_lints\!(\\1    $NEW_LINT_NAME,\n)/" "$SRC_QUERY_FILE"
    echo ' done!'
fi

echo ''
echo 'Lint created successfully! Remember to:'
echo "- implement the lint in ${LINT_FILENAME#"$TOPLEVEL/"}"
echo "- populate the test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"}"
echo "- add the expected test outputs in ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"}"
