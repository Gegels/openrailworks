@echo off
REM OpenRailWorks — Release-Build + Verknuepfung auf dem Windows-Desktop
setlocal EnableExtensions
cd /d "%~dp0\.."

where cargo >nul 2>&1
if errorlevel 1 (
    echo [Fehler] Rust/cargo nicht gefunden. Installer: https://rustup.rs
    exit /b 1
)

echo Baue OpenRailWorks Launcher ^(GUI, Release^)...
cargo build --release -p orw-launcher
if errorlevel 1 (
    echo [Fehler] Build fehlgeschlagen.
    exit /b 1
)

set "EXE=%CD%\target\release\orw-launcher.exe"
if not exist "%EXE%" (
    echo [Fehler] Datei fehlt: %EXE%
    exit /b 1
)

set "PS1=%~dp0install-launcher-shortcut.ps1"
set "LINK="
for /f "delims=" %%L in ('powershell -NoProfile -ExecutionPolicy Bypass -File "%PS1%" -ExePath "%EXE%" -WorkingDirectory "%CD%"') do set "LINK=%%L"

if errorlevel 1 (
    echo [Warnung] Verknuepfung konnte nicht erstellt werden. Starte direkt:
    echo   "%EXE%"
    exit /b 0
)

echo.
echo Fertig.
echo   Programm:  %EXE%
echo   Verknuepfung: %LINK%
echo.
echo Doppelklick auf "OpenRailWorks" auf dem Desktop oeffnet die GUI.
echo RailWorks-Pfad: im Launcher unter Einstellungen -^> Allgemein.
echo Optional: set RAILWORKS_PATH=C:\Program Files ^(x86^)\Steam\steamapps\common\RailWorks
echo.
exit /b 0
