# Erster Push nach GitHub (Windows)

## Häufige Fehler

| Meldung | Ursache | Lösung |
|---------|---------|--------|
| `fatal: not a git repository` | `git` in `C:\Users\georg` oder `System32` | Erst in einen **Clone-Ordner** wechseln (mit `.git`) |
| `Die Syntax für den Dateinamen … ist falsch` | Alle Befehle **in einer Zeile** ohne Leerzeichen | **Ein Befehl pro Zeile** + Enter (siehe unten) |
| `Cursor origin: your access token was not accepted` | PC ist **nicht** bei Cursor Origin eingeloggt | **Cursor-Clone auf Windows überspringen** — siehe „Ohne Cursor-Clone“ |

---

## Empfohlen: Ohne Cursor-Clone (nur GitHub + Cloud Agent)

Du willst **nicht** lokal entwickeln. Dann brauchst du auf dem PC **keinen** erfolgreichen Cursor-`git clone`.

**Am einfachsten:** einmal [Deploy Key](DEPLOY_KEY_EINMAL.md) auf GitHub hinterlegen → Agent schreibt: „Pushe zu GitHub.“

Alternativ (wenn Cursor **Environment-Secret** `GH_TOKEN` für Cloud Agents gesetzt ist und ein neuer Agent-Lauf läuft):

1. Secret **`GH_TOKEN`** im **Agent-Environment** (nicht nur App-Secret).
2. Dem Agenten schreiben: **„Pushe main, develop, release und Tags zu GitHub.“**
3. Auf GitHub: **Settings → Default branch → `develop`**.

Fertig — weitere Arbeit nur noch per **Cloud Agent** auf `Gegels/openrailworks`.

---

## Optional: Windows nur für den ersten Push (ohne Cursor-URL)

Wenn der Agent noch nicht pushen kann, aber du **einmalig** von Windows aus liefern willst:

**Quelle darf nicht `origin.cursor.com` sein**, wenn `origin auth login` auf dem PC fehlt.

Stattdessen nach dem ersten erfolgreichen GitHub-Push:

```cmd
cd /d %USERPROFILE%\Documents
git clone https://github.com/Gegels/openrailworks.git
```

(Danach ist GitHub voll — du brauchst den Clone nur, wenn du wirklich lokal pushen willst.)

---

## Falls du Cursor-Clone auf Windows doch nutzen willst

Zuerst Cursor CLI / Origin anmelden (nur wenn installiert):

```cmd
origin auth login
```

Dann **ein Befehl pro Zeile**:

```cmd
cd /d %USERPROFILE%\Documents
```

```cmd
git clone https://origin.cursor.com/git/georg-suchanek/tmp-c084d2fe9d243296.git openrailworks
```

```cmd
cd openrailworks
```

```cmd
git remote add github https://github.com/Gegels/openrailworks.git
```

```cmd
git push -u github main develop release --tags
```

Oder im Projektordner: `scripts\windows-push-github.bat` (nur wenn Clone schon existiert).

---

## GitHub-Login beim Push

Bei `git push` zu `github.com`:

- Browser-Anmeldung, oder  
- **Personal Access Token** als Passwort (Benutzername = dein GitHub-Name)

Classic PAT: Scope `repo`. Fine-grained: nur Repo **Gegels/openrailworks**, Contents read/write.

---

## Danach

https://github.com/Gegels/openrailworks — Branches `develop`, `release`, `main`, Tag `v0.0.1`.

Workflow: [GIT_WORKFLOW.md](GIT_WORKFLOW.md)
