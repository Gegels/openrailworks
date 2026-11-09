#!/usr/bin/env bash
# Push main + tags to GitHub (requires GH_TOKEN or gh auth).
set -euo pipefail
cd "$(dirname "$0")/.."

REMOTE_URL="${GITHUB_REMOTE:-https://github.com/Gegels/openrailworks.git}"

if [[ -n "${GH_TOKEN:-}" ]]; then
  REMOTE_URL="https://x-access-token:${GH_TOKEN}@github.com/Gegels/openrailworks.git"
fi

git remote remove github 2>/dev/null || true
git remote add github "$REMOTE_URL"

git push -u github main
git push github --tags

echo "GitHub: https://github.com/Gegels/openrailworks"
