@echo off
chcp 65001 >nul
:: Image-Viewer Windows Installer

echo =========================================
echo Image-Viewer Windows Installation
echo =========================================
echo.

set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"

:: Find executable
set "EXE_PATH="

if exist "%SCRIPT_DIR%\..\..\image-viewer.exe" (
    for %%F in ("%SCRIPT_DIR%\..\..") do set "EXE_PATH=%%~fF\image-viewer.exe"
    goto :found
)

if exist "%SCRIPT_DIR%\image-viewer.exe" (
    set "EXE_PATH=%SCRIPT_DIR%\image-viewer.exe"
    goto :found
)

if exist "%LOCALAPPDATA%\Image-Viewer\image-viewer.exe" (
    set "EXE_PATH=%LOCALAPPDATA%\Image-Viewer\image-viewer.exe"
    goto :found
)

echo [ERROR] image-viewer.exe not found!
pause
exit /b 1

:found
echo Found: %EXE_PATH%
echo.

:: Register using PowerShell inline command
echo Registering context menu...

set "PSCMD=powershell -NoProfile -ExecutionPolicy Bypass -Command \"& {$exe='%EXE_PATH%'; 'png','jpg','jpeg','gif','webp','tiff','tif','bmp','ico','heic','heif','avif' | ForEach-Object { $p='HKCU:\Software\Classes\.'+$_+'\shell\OpenWithImageViewer'; New-Item -Path $p -Force | Out-Null; Set-ItemProperty -Path $p -Name '(Default)' -Value 'Open with Image-Viewer'; Set-ItemProperty -Path $p -Name 'Icon' -Value $exe; $c=$p+'\command'; New-Item -Path $c -Force | Out-Null; Set-ItemProperty -Path $c -Name '(Default)' -Value \"`"$exe`\" \"%%1\"\"; }; Write-Host 'Done.'}\"

%PSCMD%

if %errorlevel% neq 0 (
    echo [ERROR] Registration failed.
    pause
    exit /b 1
)

echo.
echo =========================================
echo Installation completed!
echo =========================================
echo.
pause
