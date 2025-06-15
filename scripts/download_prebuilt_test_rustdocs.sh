#!/usr/bin/env bash

# check for bash
if [ -z "$BASH_VERSION" ]; then
    >&2 printf 'This script must be run using the bash shell.\n'
    exit 1
fi

set -euo pipefail

TOPLEVEL="$(git rev-parse --show-toplevel 2>/dev/null || { cd -- "$(dirname -- "${BASH_SOURCE[0]}")"/.. && pwd; })"
cd "$TOPLEVEL"

TRIPLE="$(rustc -vV | grep '^host:' | awk '{print $2}')"
VERSION="$(rustc --version | awk '{print $2}')"
HASH="$(scripts/hash_test_rustdocs_inputs.sh)"

ARTIFACT_NAME="test-rustdocs-$HASH-$TRIPLE-$VERSION"

RUNS_JSON="$(curl -s "https://api.github.com/repos/obi1kenobi/cargo-semver-checks/actions/runs?branch=main&status=success&per_page=1")"
RUN_ID="$(echo "$RUNS_JSON" | jq -r '.workflow_runs[0].id')"

ARTIFACT_URL="$(curl -s "https://api.github.com/repos/obi1kenobi/cargo-semver-checks/actions/runs/$RUN_ID/artifacts" | jq -r --arg NAME "$ARTIFACT_NAME" '.artifacts[] | select(.name==$NAME) | .archive_download_url' | head -n1)"

if [[ -z "$ARTIFACT_URL" ]]; then
    echo "No prebuilt test rustdocs found for artifact $ARTIFACT_NAME" >&2
    echo "Run ./scripts/regenerate_test_rustdocs.sh to generate them locally." >&2
    exit 1
fi

mkdir -p localdata/test_data
rm -rf localdata/test_data

curl -L "$ARTIFACT_URL" -o artifact.tgz

tar -xzf artifact.tgz -C localdata
rm artifact.tgz

