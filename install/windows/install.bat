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
    set "EXE_PATH=%SCRIPT_DIR%\..\..\image-viewer.exe"
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

:: Write PowerShell script using .NET registry methods
echo Registering context menu...

echo $exe = "%EXE_PATH%" > "%TEMP%\register_image_viewer.ps1"
echo $exts = "png","jpg","jpeg","gif","webp","tiff","tif","bmp","ico","heic","heif","avif" >> "%TEMP%\register_image_viewer.ps1"
echo foreach ($ext in $exts) { >> "%TEMP%\register_image_viewer.ps1"
echo     $path = "Software\Classes\SystemFileAssociations\." + $ext + "\shell\OpenWithImageViewer" >> "%TEMP%\register_image_viewer.ps1"
echo     $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey($path) >> "%TEMP%\register_image_viewer.ps1"
echo     $key.SetValue("", "Open with Image-Viewer") >> "%TEMP%\register_image_viewer.ps1"
echo     $key.SetValue("Icon", $exe) >> "%TEMP%\register_image_viewer.ps1"
echo     $cmdKey = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey($path + "\command") >> "%TEMP%\register_image_viewer.ps1"
echo     $cmdKey.SetValue("", ('"' + $exe + '" "%1"')) >> "%TEMP%\register_image_viewer.ps1"
echo     $cmdKey.Close() >> "%TEMP%\register_image_viewer.ps1"
echo     $key.Close() >> "%TEMP%\register_image_viewer.ps1"
echo } >> "%TEMP%\register_image_viewer.ps1"
echo Write-Host "Done." >> "%TEMP%\register_image_viewer.ps1"

powershell -NoProfile -ExecutionPolicy Bypass -File "%TEMP%\register_image_viewer.ps1"
del "%TEMP%\register_image_viewer.ps1"

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
