# GitHub monorepo

Recommended canonical repository:

**https://github.com/suchanek/openrailworks** (private or public — your choice)

> If your GitHub username differs, replace `suchanek` in URLs and scripts.

## Create the repository (once)

### Option A — `gh` with a personal access token

On a machine where you are logged in (or with `GH_TOKEN` set):

```bash
cd openrailworks
./scripts/create-github-repo.sh
```

### Option B — Cursor UI

In this agent run, use **Create repo** and name it `openrailworks`. Then add the GitHub remote:

```bash
git remote add github https://github.com/<user>/openrailworks.git
git push -u github main --tags
```

### Option C — Empty repo on github.com

1. Create **openrailworks** (no README) on GitHub.
2. Run:

```bash
git remote add github https://github.com/suchanek/openrailworks.git
git push -u github main --tags
```

## Remotes

| Remote | Purpose |
|--------|---------|
| `github` | **Primary** — issues, Actions, Releases |
| `origin` | Cursor Origin (optional mirror while migrating) |

```bash
git remote -v
origin   https://origin.cursor.com/georg-suchanek/....git
github   https://github.com/suchanek/openrailworks.git
```

## Cloud agent + `GH_TOKEN`

Hosted agents do not have `gh login` by default. To let the agent create or push to GitHub:

1. Create a fine-grained PAT with **Contents: Read/Write** on `openrailworks`.
2. In Cursor → Cloud / Secrets, add **`GH_TOKEN`** for this project (not in chat).
3. Re-run `scripts/create-github-repo.sh` in the agent.

## CI / Releases

- **CI** — `.github/workflows/ci.yml` (Windows, Linux, macOS).
- **Release** — tag `v*` triggers artifact upload via `softprops/action-gh-release`.

Workflows run on **GitHub** when the default remote is GitHub.
