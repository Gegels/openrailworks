# Nach dem Token: nächste Schritte

## Wo das Token wirkt

| Ort | Wirkung |
|-----|---------|
| **Projekt-Secret `GH_TOKEN`** (App) | Für **veröffentlichte Cursor-Apps** — oft **nicht** in der Agent-`git`-Shell |
| **Environment-Secret `GH_TOKEN`** (Dashboard) | Für **Cloud-Agent-VM** — ideal für `git push` |

Wenn der laufende Agent noch nicht pushen kann: **einmalig** Code nach GitHub (Bundle unten) **oder** neuen Agent starten nach Environment-Secret.

## Schritt A — Einmalig Code auf GitHub (empfohlen, ~2 Min.)

1. **`openrailworks.bundle`** vom Agent herunterladen (Artifacts).
2. Windows **cmd**, je **eine Zeile**:

```cmd
cd /d %USERPROFILE%\Documents
```

```cmd
git clone C:\Users\georg\Downloads\openrailworks.bundle openrailworks
```

```cmd
cd openrailworks
```

```cmd
git remote remove origin
```

```cmd
git remote add origin https://github.com/Gegels/openrailworks.git
```

```cmd
git push -u origin main develop release --tags
```

Beim Push: **derselbe PAT** wie in Cursor (`GH_TOKEN`).

3. GitHub → [Settings → Default branch](https://github.com/Gegels/openrailworks/settings) → **`develop`**.

## Schritt B — Nur noch Cloud Agent

1. **Neuer Cloud Agent** starten
2. Repository: **`Gegels/openrailworks`** (nicht Cursor-Tmp)
3. Branch: **`develop`**
4. Erste Nachricht:

```text
Lies AGENTS.md und docs/GIT_WORKFLOW.md. Arbeite auf develop.
Phase 1: orw-vfs (read-only .ap). Keine Commits auf release ohne Freigabe.
```

## Schritt C — Freigabe Release

Wenn `develop` stabil ist: GitHub → **PR** `develop` → `release` → du mergest → optional Tag `v0.0.2` auf `release`.

Siehe [GIT_WORKFLOW.md](GIT_WORKFLOW.md).
