@echo off
REM OpenRailWorks: push all branches to GitHub (run AFTER clone succeeded).
REM One double-click - runs commands sequentially.
setlocal
cd /d "%~dp0\.."
echo Working directory: %CD%
if not exist ".git" (
  echo ERROR: This folder is not a git repo. Do not run from System32.
  echo See docs\WINDOWS_FIRST_PUSH.md
  exit /b 1
)
git remote remove github 2>nul
git remote add github https://github.com/Gegels/openrailworks.git
git push -u github main develop release --tags
if errorlevel 1 (
  echo.
  echo Push failed. Use GitHub login or Personal Access Token when prompted.
  echo Or skip Windows: set GH_TOKEN in Cursor and let the Cloud Agent push.
  exit /b 1
)
echo Done. Set default branch to develop on github.com/Gegels/openrailworks/settings
pause
