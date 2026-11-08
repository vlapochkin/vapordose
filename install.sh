#!/bin/bash
# Build VaporDose from source and install it for the current user.

set -e

BIN_NAME="vapordose"

echo "Building $BIN_NAME (release)..."
cargo build --release

echo "Installing binary to ~/.local/bin..."
mkdir -p ~/.local/bin
install -m 755 "target/release/$BIN_NAME" ~/.local/bin/

# SmokeAPI binaries are looked up next to the executable (resources/) or in
# ~/.local/share/vapordose/resources. Copy the bundled ones if present.
if [ -d resources ] && [ -n "$(ls -A resources)" ]; then
    echo "Installing bundled SmokeAPI resources..."
    mkdir -p ~/.local/share/vapordose/resources
    cp -r resources/. ~/.local/share/vapordose/resources/
else
    echo "No bundled resources found; use 'Update SmokeAPI' from the app menu after first launch."
fi

echo "Installing icon..."
mkdir -p ~/.local/share/icons/hicolor/scalable/apps
install -m 644 assets/$BIN_NAME.svg ~/.local/share/icons/hicolor/scalable/apps/
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t ~/.local/share/icons/hicolor 2>/dev/null || true
fi

echo "Installing desktop entry..."
mkdir -p ~/.local/share/applications
install -m 644 "$BIN_NAME.desktop" ~/.local/share/applications/

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database ~/.local/share/applications/ || true
fi

echo "Done. Run '$BIN_NAME' or launch VaporDose from your application menu."
echo "Make sure ~/.local/bin is in your PATH."
