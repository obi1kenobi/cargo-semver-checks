#!/usr/bin/env bash

# check for bash using maximum compatibility sh syntax
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel)"
SRC_QUERY_FILE="$TOPLEVEL/src/query.rs"

# Make the script cwd-independent by always moving to the repo root first.
cd "$TOPLEVEL"

CHECK_MODE=0
if [[ "$#" -gt 1 ]]; then
    >&2 echo "Usage: scripts/sort_add_lints.sh [--check]"
    exit 1
fi

case "${1:-}" in
    "")
        ;;
    "--check")
        CHECK_MODE=1
        ;;
    "--help")
        echo "Usage: scripts/sort_add_lints.sh [--check]"
        echo "Sort add_lints!() entries and run cargo fmt."
        echo "Use --check to assert no changes were required."
        exit 0
        ;;
    *)
        >&2 echo "Usage: scripts/sort_add_lints.sh [--check]"
        exit 1
        ;;
esac

macro_start_count="$(awk '/^add_lints!\($/ { count += 1 } END { print count + 0 }' "$SRC_QUERY_FILE")"
if [[ "$macro_start_count" -ne 1 ]]; then
    >&2 echo "error: expected exactly one line matching '^add_lints!\\($' in $SRC_QUERY_FILE, found $macro_start_count."
    >&2 echo "error: this script only supports the standard one-macro layout in src/query.rs."
    exit 1
fi

macro_start_line="$(awk '/^add_lints!\($/ { print NR; exit }' "$SRC_QUERY_FILE")"
macro_end_line="$(
    awk -v start="$macro_start_line" '
        NR > start && /^\);$/ {
            print NR;
            exit;
        }
    ' "$SRC_QUERY_FILE"
)"
if [[ -z "$macro_end_line" ]]; then
    >&2 echo "error: could not find the closing ');' line for add_lints!() in $SRC_QUERY_FILE."
    exit 1
fi

entries_start_line="$((macro_start_line + 1))"
entries_end_line="$((macro_end_line - 1))"
if [[ "$entries_end_line" -lt "$entries_start_line" ]]; then
    >&2 echo "error: add_lints!() contains no entries in $SRC_QUERY_FILE."
    exit 1
fi

tmp_entries="$(mktemp)"
tmp_sorted="$(mktemp)"
tmp_rewritten="$(mktemp)"
cleanup() {
    rm -f "$tmp_entries" "$tmp_sorted" "$tmp_rewritten"
}
trap cleanup EXIT

if ! awk -v start="$entries_start_line" -v end="$entries_end_line" '
    function fail(message, line_number) {
        printf("error: %s at %s:%d\n", message, FILENAME, line_number) > "/dev/stderr";
        exit 1;
    }

    NR < start || NR > end {
        next;
    }

    {
        raw = $0;
        if (raw ~ /^[[:space:]]*$/) {
            fail("blank lines are not allowed inside add_lints!() body", NR);
        }

        entry = raw;
        sub(/^[[:space:]]+/, "", entry);
        sub(/[[:space:]]+$/, "", entry);
        if (entry !~ /,[[:space:]]*$/) {
            fail("each add_lints!() row must end with a trailing comma", NR);
        }
        sub(/,[[:space:]]*$/, "", entry);

        if (entry ~ /^[A-Za-z_][A-Za-z0-9_]*$/) {
            kind = 1;
            key = entry;
        } else if (match(entry, /^\(([A-Za-z_][A-Za-z0-9_]*)[[:space:]]*,.*\)$/, captures)) {
            kind = 0;
            key = captures[1];
        } else {
            fail("unsupported row format; expected `ident,` or `(ident, ...),` with one item per line and no inline comments", NR);
        }

        printf("%d\t%s\t%s\n", kind, key, entry);
    }
' "$SRC_QUERY_FILE" >"$tmp_entries"; then
    exit 1
fi

if [[ ! -s "$tmp_entries" ]]; then
    >&2 echo "error: no lint entries were parsed from add_lints!() in $SRC_QUERY_FILE."
    exit 1
fi

LC_ALL=C sort -t $'\t' -k1,1n -k2,2 "$tmp_entries" >"$tmp_sorted"

{
    head -n "$macro_start_line" "$SRC_QUERY_FILE"
    awk -F $'\t' '{ printf "    %s,\n", $3 }' "$tmp_sorted"
    tail -n +"$macro_end_line" "$SRC_QUERY_FILE"
} >"$tmp_rewritten"

mv "$tmp_rewritten" "$SRC_QUERY_FILE"

cargo fmt

if [[ "$CHECK_MODE" -eq 1 ]]; then
    git diff --exit-code -- src/query.rs
fi
