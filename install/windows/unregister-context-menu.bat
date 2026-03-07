@echo off
chcp 65001 >nul
:: OAS-Image-Viewer Context Menu Unregistration Script

echo ==========================================
echo OAS-Image-Viewer Context Menu Unregistration
echo ==========================================
echo.

echo Unregistering...

:: Write PowerShell script to remove registry keys
echo $exts = "png","jpg","jpeg","gif","webp","tiff","tif","bmp","ico","heic","heif","avif" > "%TEMP%\unregister_image_viewer.ps1"
echo foreach ($ext in $exts) { >> "%TEMP%\unregister_image_viewer.ps1"
echo     $path = "Software\Classes\SystemFileAssociations\." + $ext + "\shell\OpenWithOASImageViewer" >> "%TEMP%\unregister_image_viewer.ps1"
echo     try { >> "%TEMP%\unregister_image_viewer.ps1"
echo         [Microsoft.Win32.Registry]::CurrentUser.DeleteSubKeyTree($path, $false) >> "%TEMP%\unregister_image_viewer.ps1"
echo         Write-Host "Removed:" $ext >> "%TEMP%\unregister_image_viewer.ps1"
echo     } catch { >> "%TEMP%\unregister_image_viewer.ps1"
echo         Write-Host "Not found:" $ext >> "%TEMP%\unregister_image_viewer.ps1"
echo     } >> "%TEMP%\unregister_image_viewer.ps1"
echo } >> "%TEMP%\unregister_image_viewer.ps1"
echo Write-Host "Done." >> "%TEMP%\unregister_image_viewer.ps1"

powershell -NoProfile -ExecutionPolicy Bypass -File "%TEMP%\unregister_image_viewer.ps1"
del "%TEMP%\unregister_image_viewer.ps1"

echo.
echo ==========================================
echo Unregistration completed!
echo ==========================================
echo.
pause
