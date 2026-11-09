#!/usr/bin/env bash
# Merge develop into release after human approval. Prefer GitHub PR instead.
set -euo pipefail
cd "$(dirname "$0")/.."

git fetch origin develop release 2>/dev/null || git fetch --all

git checkout release
git merge --no-ff develop -m "Release: promote develop into release"

echo "Merged develop → release locally. Run: git push origin release"
echo "Prefer opening a PR on GitHub (develop → release) for audit trail."
