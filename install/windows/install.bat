@echo off
chcp 65001 >nul
:: Image-Viewer Windows Installer
:: Universal installer for Windows 7/8/10/11 - no admin rights needed

setlocal EnableDelayedExpansion

echo =========================================
echo Image-Viewer Windows Installation
echo =========================================
echo.

:: Get script directory
set "SCRIPT_DIR=%~dp0"
set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"

:: Try to find image-viewer.exe in various locations
set "EXE_FOUND=0"

:: Check 1: Same directory as script
if exist "%SCRIPT_DIR%\image-viewer.exe" (
    set "EXE_PATH=%SCRIPT_DIR%\image-viewer.exe"
    set "EXE_FOUND=1"
    goto :found_exe
)

:: Check 2: Parent directory (for install/windows/ structure)
if exist "%SCRIPT_DIR%\..\..\image-viewer.exe" (
    for %%F in ("%SCRIPT_DIR%\..\..") do set "EXE_PATH=%%~fF\image-viewer.exe"
    set "EXE_FOUND=1"
    goto :found_exe
)

:: Check 3: Three levels up (for green version structure)
if exist "%SCRIPT_DIR%\..\..\..\image-viewer.exe" (
    for %%F in ("%SCRIPT_DIR%\..\..\..") do set "EXE_PATH=%%~fF\image-viewer.exe"
    set "EXE_FOUND=1"
    goto :found_exe
)

:: Check 4: Standard LocalAppData location
if exist "%LOCALAPPDATA%\Image-Viewer\image-viewer.exe" (
    set "EXE_PATH=%LOCALAPPDATA%\Image-Viewer\image-viewer.exe"
    set "EXE_FOUND=1"
    goto :found_exe
)

:: Check 5: Source build location
if exist "%SCRIPT_DIR%\..\..\target\release\image-viewer.exe" (
    for %%F in ("%SCRIPT_DIR%\..\..\target\release") do set "EXE_PATH=%%~fF\image-viewer.exe"
    set "EXE_FOUND=1"
    goto :found_exe
)

:: Not found
if %EXE_FOUND%==0 (
    echo [ERROR] image-viewer.exe not found!
    pause
    exit /b 1
)

:found_exe
echo Found executable: %EXE_PATH%
echo.

:: Register context menu
echo Registering context menu...
echo.

:: Use PowerShell for reliable registry operations
echo Creating registry entries...

powershell -NoProfile -ExecutionPolicy Bypass -Command "\
\$exePath = '%EXE_PATH%';
\$formats = @('png', 'jpg', 'jpeg', 'gif', 'webp', 'tiff', 'tif', 'bmp', 'ico', 'heic', 'heif', 'avif');

foreach (\$fmt in \$formats) {
    \$regPath = \"HKCU:\Software\Classes\.$fmt\shell\OpenWithImageViewer\";
    New-Item -Path \$regPath -Force | Out-Null;
    Set-ItemProperty -Path \$regPath -Name '(Default)' -Value 'Open with Image-Viewer';
    Set-ItemProperty -Path \$regPath -Name 'Icon' -Value \$exePath;
    
    \$cmdPath = \"\$regPath\command\";
    New-Item -Path \$cmdPath -Force | Out-Null;
    Set-ItemProperty -Path \$cmdPath -Name '(Default)' -Value \"\`\"\$exePath\`\" \"%%1\"\";
}

Write-Host 'Registry entries created successfully.';
"

if %errorlevel% neq 0 (
    echo [ERROR] Failed to create registry entries.
    pause
    exit /b 1
)

echo.
echo =========================================
echo Installation completed successfully!
echo =========================================
echo.
echo Image-Viewer path: %EXE_PATH%
echo.
echo You can now right-click on image files and select:
echo   "Open with Image-Viewer"
echo.
echo NOTE for Windows 11 users:
echo If you don't see the menu in the modern context menu,
echo click "Show more options" to see the traditional menu.
echo.

pause
