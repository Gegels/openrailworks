# Erster Push nach GitHub (Windows)

## Fehler: `fatal: not a git repository`

`git` muss in einem Ordner laufen, in dem bereits ein **`.git`** Verzeichnis existiert (Clone oder `git init`).  
`C:\Users\georg` ist **kein** Projektordner.

## Empfohlen: Code von Cursor klonen, dann zu GitHub pushen

Der vollständige Stand liegt noch auf **Cursor Origin** (bis er auf GitHub ist).

### 1. Ordner wählen und klonen

**Eingabeaufforderung** oder **PowerShell**:

```cmd
cd %USERPROFILE%\Documents
git clone https://origin.cursor.com/git/georg-suchanek/tmp-c084d2fe9d243296.git openrailworks
cd openrailworks
```

Beim Klonen kann nach **Cursor-/Git-Anmeldung** gefragt werden (Browser oder Token).

### 2. GitHub als Ziel eintragen und pushen

```cmd
git remote add github https://github.com/Gegels/openrailworks.git
git push -u github main --tags
```

GitHub fragt nach Login (Browser, **gh auth login**, oder Personal Access Token als Passwort bei HTTPS).

### 3. Prüfen

Im Browser: https://github.com/Gegels/openrailworks — dort sollten `README.md`, `crates/`, `docs/`, Tag `v0.0.1` sichtbar sein.

---

## Alternative: Projekt bereits in Cursor auf dem PC

Wenn du den Agent-Ordner lokal hast:

```cmd
cd PFAD\ZUM\openrailworks
git status
git remote -v
git push -u https://github.com/Gegels/openrailworks.git main --tags
```

`PFAD\ZUM\openrailworks` durch den echten Workspace-Pfad ersetzen (in Cursor: Explorer → Ordnerpfad).

---

## Nur leeres GitHub-Repo geklont?

`git clone https://github.com/Gegels/openrailworks.git` liefert **noch keinen** OpenRailWorks-Code (Repo war leer). Zuerst Cursor-Repo klonen (Schritt 1 oben) oder Inhalt aus einem Cursor-Export übernehmen.

---

## GitHub CLI (optional)

```powershell
winget install GitHub.cli
gh auth login
cd %USERPROFILE%\Documents\openrailworks
gh repo sync
```

Nach dem ersten manuellen Push reicht für künftige Agenten: **Cloud-Agent von `Gegels/openrailworks` starten**.
