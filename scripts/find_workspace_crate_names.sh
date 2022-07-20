#!/usr/bin/env bash

# Script requirements:
# - jq

# Fail on first error, on undefined variables, and on failures in pipelines.
set -euo pipefail

# Go to the repo root directory.
cd "$(git rev-parse --show-toplevel)"

cargo metadata --format-version 1 | \
    jq --exit-status -r \
        '.workspace_members[] as $key | .packages[] | select(.id == $key) | .name'
