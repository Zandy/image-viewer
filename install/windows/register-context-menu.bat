@echo off
chcp 65001 >nul
:: Image-Viewer Context Menu Registration Script
:: Universal for Windows 7/8/10/11 - no admin rights needed

setlocal EnableDelayedExpansion

echo ==========================================
echo Image-Viewer Context Menu Registration
echo ==========================================
echo.

:: Get script directory (e.g., install\windows\
set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"

echo Script directory: %SCRIPT_DIR%
echo.

:: Try to find image-viewer.exe in various locations
set "EXE_FOUND=0"
set "EXE_PATH="

:: Check 1: Parent of parent directory (for install\windows\ structure)
:: SCRIPT_DIR is install\windows\, so ..\.. is the root
echo Checking: %SCRIPT_DIR%\..\..\image-viewer.exe
if exist "%SCRIPT_DIR%\..\..\image-viewer.exe" (
    for %%F in ("%SCRIPT_DIR%\..\..") do set "ROOT_DIR=%%~fF"
    set "EXE_PATH=%ROOT_DIR%\image-viewer.exe"
    set "EXE_FOUND=1"
    echo [OK] Found at: %EXE_PATH%
    goto :found_exe
)

:: Check 2: Same directory as script (rare case)
echo Checking: %SCRIPT_DIR%\image-viewer.exe
if exist "%SCRIPT_DIR%\image-viewer.exe" (
    set "EXE_PATH=%SCRIPT_DIR%\image-viewer.exe"
    set "EXE_FOUND=1"
    echo [OK] Found at: %EXE_PATH%
    goto :found_exe
)

:: Check 3: Standard LocalAppData location
echo Checking: %LOCALAPPDATA%\Image-Viewer\image-viewer.exe
if exist "%LOCALAPPDATA%\Image-Viewer\image-viewer.exe" (
    set "EXE_PATH=%LOCALAPPDATA%\Image-Viewer\image-viewer.exe"
    set "EXE_FOUND=1"
    echo [OK] Found at: %EXE_PATH%
    goto :found_exe
)

:: Not found
if %EXE_FOUND%==0 (
    echo.
    echo [ERROR] image-viewer.exe not found!
    echo.
    echo Please ensure you have downloaded the correct package.
    echo The exe should be in the same folder as 'install' directory.
    echo.
    echo Expected structure:
    echo   image-viewer-v0.2.0-windows-x86_64/
    echo   ├── image-viewer.exe      ^<-- This file
    echo   └── install/
    echo       └── windows/
    echo           └── register-context-menu.bat  ^<-- You are here
    echo.
    pause
    exit /b 1
)

:found_exe
echo.
echo Executable: %EXE_PATH%
echo.

:: Register context menu using PowerShell
echo Registering context menu...
echo.

powershell -NoProfile -ExecutionPolicy Bypass -Command "\
\$exePath = '%EXE_PATH%'; \
\$formats = @('png', 'jpg', 'jpeg', 'gif', 'webp', 'tiff', 'tif', 'bmp', 'ico', 'heic', 'heif', 'avif'); \

foreach (\$fmt in \$formats) { \
    \$regPath = \"HKCU:\Software\Classes\.$fmt\shell\OpenWithImageViewer\"; \
    New-Item -Path \$regPath -Force ^| Out-Null; \
    Set-ItemProperty -Path \$regPath -Name '(Default)' -Value 'Open with Image-Viewer'; \
    Set-ItemProperty -Path \$regPath -Name 'Icon' -Value \$exePath; \
    \
    \$cmdPath = \"\$regPath\command\"; \
    New-Item -Path \$cmdPath -Force ^| Out-Null; \
    Set-ItemProperty -Path \$cmdPath -Name '(Default)' -Value \"\`\"\$exePath\`\" \"%%1\"\"; \
}\

Write-Host 'Context menu registered successfully.'; \
"

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Failed to register context menu.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo Registration successful!
echo ==========================================
echo.
echo You can now right-click on image files and select:
echo   "Open with Image-Viewer"
echo.
echo NOTE for Windows 11 users:
echo If you don't see the menu in the modern context menu,
echo click "Show more options" or press Shift+F10
echo.

pause
