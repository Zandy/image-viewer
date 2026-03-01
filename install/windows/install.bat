@echo off
chcp 65001 >nul
:: Image-Viewer Windows Installer
:: This script installs Image-Viewer and registers it in the context menu
:: Supports Windows 10 and Windows 11

setlocal EnableDelayedExpansion

echo =========================================
echo Image-Viewer Windows Installation
echo =========================================
echo.

:: Set installation directory
:: Use LocalAppData for user-specific installation (recommended for Windows 10/11)
set "INSTALL_DIR=%LOCALAPPDATA%\Image-Viewer"
set "SOURCE_DIR=%~dp0"

echo Installation directory: %INSTALL_DIR%
echo.

:: Create installation directory
if not exist "%INSTALL_DIR%" (
    echo Creating installation directory...
    mkdir "%INSTALL_DIR%"
)

:: Copy executable (assuming it's built)
if exist "%SOURCE_DIR%\..\..\target\release\image-viewer.exe" (
    echo Copying image-viewer.exe...
    copy /Y "%SOURCE_DIR%\..\..\target\release\image-viewer.exe" "%INSTALL_DIR%\" >nul
) else if exist "%SOURCE_DIR%\image-viewer.exe" (
    echo Copying image-viewer.exe...
    copy /Y "%SOURCE_DIR%\image-viewer.exe" "%INSTALL_DIR%\" >nul
) else (
    echo Warning: image-viewer.exe not found.
    echo Please build the project first with: cargo build --release
    pause
    exit /b 1
)

:: Detect Windows version
for /f "tokens=4-5 delims=. " %%i in ('ver') do set VERSION=%%i.%%j

:: Register context menu
:: Use the new Windows 10/11 compatible registry file
if exist "%SOURCE_DIR%\register-context-menu-win10-win11.reg" (
    echo.
    echo Detected Windows %VERSION%
    echo Using modern registry registration (Windows 10/11 compatible)...
    echo.
    regedit /s "%SOURCE_DIR%\register-context-menu-win10-win11.reg"
) else (
    echo.
    echo Using legacy registry registration...
    echo Note: This requires Administrator privileges on some systems.
    echo.
    regedit /s "%SOURCE_DIR%\register-context-menu.reg"
)

if %errorLevel% equ 0 (
    echo.
    echo =========================================
    echo Installation completed successfully!
    echo =========================================
    echo.
    echo Image-Viewer installed to: %INSTALL_DIR%
    echo.
    echo You can now right-click on image files to open with Image-Viewer.
    echo.
    echo Supported formats: PNG, JPG, JPEG, GIF, WebP, TIFF, TIF, BMP, ICO, HEIC, HEIF, AVIF
    echo.
) else (
    echo.
    echo Error: Failed to register context menu.
    echo.
    echo Troubleshooting:
    echo 1. Make sure you have write access to the registry
    echo 2. Try running as Administrator if the above fails
    echo 3. Check if the .reg files exist in: %SOURCE_DIR%
    echo.
)

pause
