@echo off
setlocal
cd /d "%~dp0\.."
cargo build --release --workspace
if errorlevel 1 exit /b 1
cargo test --workspace
if errorlevel 1 exit /b 1
echo Artifacts: target\release\orw-catalog.exe
