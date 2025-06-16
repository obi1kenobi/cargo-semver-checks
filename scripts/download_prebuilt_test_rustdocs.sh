#!/usr/bin/env bash

# check for bash
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

set -euo pipefail

# 1. Is `gh` available?
if ! command -v gh >/dev/null 2>&1; then
    >&2 echo "The GitHub CLI ('gh') is required but was not found in your PATH."
    >&2 echo "If you use homebrew, you can install it with:"
    >&2 echo "    brew install gh"
    >&2 echo "Then authenticate with:"
    >&2 echo "    gh auth login"
    exit 1
fi

# 2. Is the user logged in?
# `gh auth status` exits non‑zero when not authenticated, which would normally
# stop the script because of `set -e`.  Capture the output instead and inspect it.
GH_STATUS="$(gh auth status 2>&1 || true)"
if ! grep -q 'Logged in to github\.com account' <<<"$GH_STATUS"; then
    >&2 echo "The GitHub CLI is installed but not authenticated."
    >&2 echo "Run:"
    >&2 echo "    gh auth login"
    >&2 echo "and try again."
    exit 1
fi

TOPLEVEL="$(git rev-parse --show-toplevel 2>/dev/null || { cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/.. && pwd; })"
cd "$TOPLEVEL"

TRIPLE="$(rustc -vV | grep '^host:' | awk '{print $2}')"
VERSION="$(rustc --version | awk '{print $2}')"
HASH="$(scripts/hash_test_rustdocs_inputs.sh)"

ARTIFACT_NAME="test-rustdocs-$HASH-$TRIPLE-$VERSION"

RUNS_JSON="$(curl -s "https://api.github.com/repos/obi1kenobi/cargo-semver-checks/actions/workflows/ci.yml/runs?branch=main&status=success&per_page=1")"
RUN_ID="$(echo "$RUNS_JSON" | jq -r '.workflow_runs[0].id')"

ARTIFACT_URL="$(curl -s "https://api.github.com/repos/obi1kenobi/cargo-semver-checks/actions/runs/$RUN_ID/artifacts" | jq -r --arg NAME "$ARTIFACT_NAME" '.artifacts[] | select(.name==$NAME) | .archive_download_url' | head -n1)"

if [[ -z "$ARTIFACT_URL" ]]; then
    echo "No prebuilt test rustdocs found for artifact $ARTIFACT_NAME" >&2
    echo "Run ./scripts/regenerate_test_rustdocs.sh to generate them locally." >&2
    exit 1
fi

mkdir -p localdata

gh api "$ARTIFACT_URL"

rm -rf localdata/test_data
mkdir -p localdata/test_data/
unzip -q localdata/artifact.zip -d localdata/test_data/
rm localdata/artifact.zip
