#!/usr/bin/env bash
# Creates github.com/<owner>/openrailworks and pushes this monorepo.
set -euo pipefail

OWNER="${GITHUB_OWNER:-suchanek}"
REPO_NAME="${GITHUB_REPO:-openrailworks}"
VISIBILITY="${GITHUB_VISIBILITY:-private}"

cd "$(dirname "$0")/.."

if ! command -v gh >/dev/null 2>&1; then
  echo "Install GitHub CLI: https://cli.github.com/"
  exit 1
fi

if ! gh auth status -h github.com >/dev/null 2>&1; then
  if [[ -z "${GH_TOKEN:-}" ]]; then
    echo "Not logged in to GitHub. Run: gh auth login"
    echo "Or set GH_TOKEN (PAT with repo scope) for cloud agents."
    exit 1
  fi
  export GH_TOKEN
fi

FULL="${OWNER}/${REPO_NAME}"

if gh repo view "$FULL" >/dev/null 2>&1; then
  echo "Repository $FULL already exists."
else
  echo "Creating $FULL ($VISIBILITY)..."
  gh repo create "$FULL" \
    --"${VISIBILITY}" \
    --description "Open-source Train Simulator Classic engine (openOMSI-style); monorepo" \
    --source . \
    --remote github \
    --push
  echo "Created and pushed."
  exit 0
fi

if git remote get-url github >/dev/null 2>&1; then
  :
else
  gh repo set-default "$FULL"
  git remote add github "https://github.com/${FULL}.git"
fi

git push -u github main
git push github --tags || true
echo "Pushed to https://github.com/${FULL}"
