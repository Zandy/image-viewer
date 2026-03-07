#!/bin/bash

# OAS-Image-Viewer Linux Uninstaller

set -e

echo "========================================="
echo "OAS-Image-Viewer Linux Uninstallation"
echo "========================================="
echo ""

# Detect installation type
USER_BIN="$HOME/.local/bin/oas-image-viewer"
USER_DESKTOP="$HOME/.local/share/applications/oas-image-viewer.desktop"
SYSTEM_BIN="/usr/local/bin/oas-image-viewer"
SYSTEM_DESKTOP="/usr/share/applications/oas-image-viewer.desktop"

if [[ -f "$USER_BIN" ]]; then
    INSTALL_TYPE="user"
    BIN_DIR="$HOME/.local/bin"
    APP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
elif [[ -f "$SYSTEM_BIN" ]]; then
    INSTALL_TYPE="system"
    BIN_DIR="/usr/local/bin"
    APP_DIR="/usr/share/applications"
    ICON_DIR="/usr/share/icons/hicolor/256x256/apps"
else
    echo "OAS-Image-Viewer not found. Nothing to uninstall."
    exit 0
fi

echo "Detected installation type: $INSTALL_TYPE"
echo ""

read -p "Are you sure you want to uninstall OAS-Image-Viewer? [y/N]: " confirm
if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
    echo "Uninstallation cancelled."
    exit 0
fi

echo "Removing files..."

if [[ "$INSTALL_TYPE" == "system" ]]; then
    sudo rm -f "$BIN_DIR/oas-image-viewer"
    sudo rm -f "$APP_DIR/oas-image-viewer.desktop"
    sudo rm -f "$ICON_DIR/oas-image-viewer.png"
    
    if command -v update-desktop-database &> /dev/null; then
        sudo update-desktop-database "$APP_DIR" 2>/dev/null || true
    fi
else
    rm -f "$BIN_DIR/oas-image-viewer"
    rm -f "$APP_DIR/oas-image-viewer.desktop"
    rm -f "$ICON_DIR/oas-image-viewer.png"
    
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$APP_DIR" 2>/dev/null || true
    fi
fi

echo ""
echo "========================================="
echo "Uninstallation completed!"
echo "========================================="
