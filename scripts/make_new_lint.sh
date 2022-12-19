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
if [[ -f "$LINT_FILENAME" ]]; then
    echo "A lint named '$NEW_LINT_NAME' appears to have already been defined in $LINT_FILENAME"
    exit 1
fi
echo -n "Creating the lint definition file ${LINT_FILENAME#"$TOPLEVEL/"} ..."
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

# Make the test crates.
NEW_LINT_TEST_CRATES_DIR="$TEST_CRATES_DIR/$NEW_LINT_NAME"
echo -n "Creating test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"} ..."
cp -R "$TEST_CRATES_DIR/template" "$NEW_LINT_TEST_CRATES_DIR"
sed -i'' "s/template/$NEW_LINT_NAME/g" "$NEW_LINT_TEST_CRATES_DIR/old/Cargo.toml"
sed -i'' "s/template/$NEW_LINT_NAME/g" "$NEW_LINT_TEST_CRATES_DIR/new/Cargo.toml"
echo ' done!'

# Add the test outputs file.
NEW_TEST_OUTPUT_FILE="$TEST_OUTPUTS_DIR/$NEW_LINT_NAME.output.ron"
echo -n "Creating the test outputs file ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"} ..."
cat <<EOF >"$NEW_TEST_OUTPUT_FILE"
[
    "./test_crates/$NEW_LINT_NAME/": [
        // TODO
    ]
]
EOF
echo ' done!'

# Add the new lint to the `add_lints!()` macro.
echo -n "Registering the new lint in src/query.rs ..."
# The -z flag allows us to process newline characters with sed.
sed -i'' -z "s/add_lints!(/add_lints!(\n    $NEW_LINT_NAME,/" "$SRC_QUERY_FILE"
echo ' done!'

echo ''
echo 'Lint created successfully! Remember to:'
echo "- implement the lint in ${LINT_FILENAME#"$TOPLEVEL/"}"
echo "- populate the test crates in ${NEW_LINT_TEST_CRATES_DIR#"$TOPLEVEL/"}"
echo "- add the expected test outputs in ${NEW_TEST_OUTPUT_FILE#"$TOPLEVEL/"}"
