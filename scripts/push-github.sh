#!/usr/bin/env bash
# Push main + tags to GitHub (GH_TOKEN, gh auth, or SSH deploy key).
set -euo pipefail
cd "$(dirname "$0")/.."

DEPLOY_KEY="${GITHUB_DEPLOY_KEY:-.deploy/github_deploy}"
REPO="${GITHUB_REPO_SLUG:-Gegels/openrailworks}"

git remote remove github 2>/dev/null || true

if [[ -n "${GH_TOKEN:-}" ]]; then
  git remote add github "https://x-access-token:${GH_TOKEN}@github.com/${REPO}.git"
elif [[ -f "$DEPLOY_KEY" ]]; then
  git remote add github "git@github.com:${REPO}.git"
  export GIT_SSH_COMMAND="ssh -i ${DEPLOY_KEY} -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new"
else
  git remote add github "https://github.com/${REPO}.git"
fi

git push -u github main
git push github --tags

echo "GitHub: https://github.com/${REPO}"
