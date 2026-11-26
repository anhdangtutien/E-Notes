@echo off
title E-Notes English Learning
echo ========================================================
echo       KHOI DONG SO TAY TIENG ANH E-NOTES (RUST)
echo ========================================================
echo.
echo Dang mo trinh duyet tai: http://localhost:8080 ...
start http://localhost:8080
echo.
echo Server dang hoat dong. De dung server, nhan Ctrl + C hoac dong cua so nay.
echo.
if exist E-Notes.exe (
    E-Notes.exe
) else if exist target\release\enotes.exe (
    target\release\enotes.exe
) else (
    cargo run --release
)
pause
