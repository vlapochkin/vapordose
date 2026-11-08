#!/bin/bash

set -e

APP_NAME="VaporDose"
BIN_NAME="vapordose"
APP_DIR="AppDir"

echo "Building $APP_NAME..."
cargo build --release

echo "Preparing AppDir..."
rm -rf $APP_DIR
mkdir -p $APP_DIR/usr/bin
mkdir -p $APP_DIR/usr/lib
mkdir -p $APP_DIR/usr/share/applications
mkdir -p $APP_DIR/usr/share/icons/hicolor/scalable/apps

# Copy binary
cp target/release/$BIN_NAME $APP_DIR/usr/bin/

# Ensure resources are present and copy them to /usr/bin/resources (where code expects them)
if [ ! -d "resources" ] || [ -z "$(ls -A resources)" ]; then
    echo "Resources missing. Downloading SmokeAPI..."
    mkdir -p resources
    LATEST_RELEASE=$(curl -s https://api.github.com/repos/acidicoala/SmokeAPI/releases/latest)
    DOWNLOAD_URL=$(echo $LATEST_RELEASE | grep -o 'https://github.com/acidicoala/SmokeAPI/releases/download/[^"]*.zip' | head -n 1)
    wget -q -O smokeapi.zip "$DOWNLOAD_URL"
    unzip -o smokeapi.zip -d resources
    rm smokeapi.zip
    TAG_NAME=$(echo $LATEST_RELEASE | grep -o '"tag_name": "[^"]*"' | head -n 1 | cut -d'"' -f4)
    echo "$TAG_NAME" > resources/version.txt
fi
cp -r resources $APP_DIR/usr/bin/

# Icon
curl -L -o $APP_DIR/usr/share/icons/hicolor/scalable/apps/$BIN_NAME.svg https://raw.githubusercontent.com/GNOME/adwaita-icon-theme/master/Adwaita/scalable/apps/preferences-system-symbolic.svg

# Desktop file
cp $BIN_NAME.desktop $APP_DIR/usr/share/applications/$BIN_NAME.desktop

# Create a custom AppRun script to avoid library conflicts
cat > $APP_DIR/AppRun <<EOF
#!/bin/sh
SELF=\$(readlink -f "\$0")
HERE=\$(dirname "\$SELF")
export PATH="\$HERE/usr/bin:\$PATH"
export LD_LIBRARY_PATH="\$HERE/usr/lib:\$LD_LIBRARY_PATH"
export XDG_DATA_DIRS="\$HERE/usr/share:\$XDG_DATA_DIRS"
exec "\$HERE/usr/bin/$BIN_NAME" "\$@"
EOF
chmod +x $APP_DIR/AppRun

# Download linuxdeploy if missing
if [ ! -f linuxdeploy-x86_64.AppImage ]; then
    curl -L -o linuxdeploy-x86_64.AppImage https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
    chmod +x linuxdeploy-x86_64.AppImage
fi

# Run linuxdeploy with NO_STRIP and excluding problematic libs
export OUTPUT="$APP_NAME-x86_64.AppImage"
export APPIMAGE_EXTRACT_AND_RUN=1
export NO_STRIP=1

# We use linuxdeploy to bundle only essential app dependencies
./linuxdeploy-x86_64.AppImage --appdir $APP_DIR \
    --output appimage \
    --desktop-file $APP_DIR/usr/share/applications/$BIN_NAME.desktop

echo "AppImage recreated: $OUTPUT"
