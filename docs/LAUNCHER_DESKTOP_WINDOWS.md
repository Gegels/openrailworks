# Launcher auf dem Windows-Desktop

OpenRailWorks hat **keine Excel-Oberfläche**, sondern ein natives **Desktop-Programm (EXE + GUI)** — vergleichbar mit dem openOMSI-Launcher.

## Schnellstart (einmalig)

1. **Rust** installieren (falls noch nicht): https://rustup.rs  
2. Repo holen und Branch mit Launcher:
   ```cmd
   git clone https://github.com/Gegels/openrailworks.git
   cd openrailworks
   git checkout cursor/phase1-launcher-af8b
   ```
   *(oder nach Merge von PR #1: `git checkout develop` und `git pull`)*  
3. **Desktop-Verknüpfung erstellen** (im Repo-Root):
   ```cmd
   scripts\install-launcher-desktop.cmd
   ```
4. Auf dem Desktop **„OpenRailWorks“** doppelklicken → Fenster mit linker Navigation und **GameCam-Vorschau** rechts.

**OneDrive-Desktop:** Der Installer ermittelt den echten Desktop-Ordner automatisch (nicht nur `C:\Users\…\Desktop`). Wenn die Verknüpfung fehlschlägt: `target\release\orw-launcher.exe` per Rechtsklick → Verknüpfung erstellen.

## Manuell ohne Skript

```cmd
cargo build --release -p orw-launcher
target\release\orw-launcher.exe
```

Doppelklick auf `orw-launcher.exe` im Explorer funktioniert ebenso (Pfad: `target\release\`).

## RailWorks finden

Im Launcher: **Allgemein** → Ordner  
`C:\Program Files (x86)\Steam\steamapps\common\RailWorks`  
(oder dein Steam-Pfad)

Alternativ vor dem Start in CMD:

```cmd
set RAILWORKS_PATH=C:\Program Files (x86)\Steam\steamapps\common\RailWorks
```

## Nur Kommandozeile (ohne Fenster)

```cmd
set ORW_LAUNCHER_CLI=1
target\release\orw-launcher.exe doctor
```

Ohne `ORW_LAUNCHER_CLI` und ohne Unterbefehl startet **immer die GUI**.

## CI-Binary (ohne lokalen Build)

GitHub Actions → Workflow **CI** → letzter grüner Lauf auf `develop` / PR-Branch → Artefakt **`orw-launcher-windows-latest`** enthält `orw-launcher.exe`. Entpacken und Verknüpfung manuell auf die EXE legen.
