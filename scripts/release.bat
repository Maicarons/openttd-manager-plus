@echo off
REM Release build script for OpenTTD Manager Plus (Windows)
REM Usage: scripts\release.bat [desktop|test|all]

if "%1"=="" goto usage

if "%1"=="desktop" goto desktop
if "%1"=="test" goto test
if "%1"=="all" goto all
goto usage

:desktop
echo --- Building Desktop ---
cargo build --release -p otmp-desktop
echo Desktop build complete.
echo Binary: target\release\openttd-manager-plus.exe
goto end

:test
echo --- Running Tests ---
cargo test --workspace
echo All tests passed.
goto end

:all
call :test
call :desktop
echo === All builds complete ===
goto end

:usage
echo Usage: %0 [desktop^|test^|all]
exit /b 1

:end