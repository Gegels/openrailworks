@echo off
setlocal
cd /d "%~dp0\.."
cargo build --release --workspace
if errorlevel 1 exit /b 1
cargo test --workspace
if errorlevel 1 exit /b 1
echo Artifacts:
echo   target\release\orw-launcher.exe   ^(GUI — Doppelklick^)
echo   target\release\orw-catalog.exe    ^(CLI-Diagnose^)
echo Desktop shortcut: scripts\install-launcher-desktop.cmd
