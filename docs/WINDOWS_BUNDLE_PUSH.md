# Einmaliger Push mit Git-Bundle (ohne Cursor-Login)

Wenn `git clone` von `origin.cursor.com` mit *access token was not accepted* fehlschlägt:

1. **Bundle** `openrailworks.bundle` aus dem Cloud-Agent-Download / Artifacts laden.
2. In **cmd** — **ein Befehl pro Zeile**:

```cmd
cd /d %USERPROFILE%\Documents
```

```cmd
git clone openrailworks.bundle openrailworks
```

(Pfad zur `.bundle`-Datei anpassen, z. B. `Downloads\openrailworks.bundle`.)

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

Bei GitHub-Login: **Personal Access Token** als Passwort (Benutzername = `Gegels` oder dein GitHub-Login).

3. Auf https://github.com/Gegels/openrailworks/settings → **Default branch** = **`develop`**.

Hinweis: `GH_TOKEN` in Cursor gilt für **veröffentlichte Apps**; die Agent-VM nutzt es oft nicht für `git push`. Dieser Bundle-Weg braucht nur **GitHub**-Anmeldung.
