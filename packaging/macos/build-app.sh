#!/usr/bin/env bash
# Build zen-markdown.app from the release binary (macOS only).
# Usage: ./packaging/macos/build-app.sh [--install]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

APP_NAME="zen-markdown"
APP="target/${APP_NAME}.app"
PKG="packaging/macos"

echo "==> building release binary"
cargo build --release

echo "==> generating icon"
python3 "$PKG/make-icon.py" "$PKG/icon-1024.png" "assets/fonts/GeistMono.ttf"

echo "==> building .icns"
ICONSET="$PKG/AppIcon.iconset"
rm -rf "$ICONSET"
mkdir -p "$ICONSET"
for s in 16 32 128 256 512; do
	sips -z $s $s        "$PKG/icon-1024.png" --out "$ICONSET/icon_${s}x${s}.png"      >/dev/null
	sips -z $((s*2)) $((s*2)) "$PKG/icon-1024.png" --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$PKG/AppIcon.icns"

echo "==> assembling $APP"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$PKG/Info.plist" "$APP/Contents/Info.plist"
cp "$PKG/AppIcon.icns" "$APP/Contents/Resources/AppIcon.icns"
cp "target/release/$APP_NAME" "$APP/Contents/MacOS/$APP_NAME"
chmod +x "$APP/Contents/MacOS/$APP_NAME"

# clear quarantine so it opens without Gatekeeper friction (local build)
xattr -cr "$APP" 2>/dev/null || true

echo "==> built $APP"

if [[ "${1:-}" == "--install" ]]; then
	echo "==> installing to /Applications"
	rm -rf "/Applications/${APP_NAME}.app"
	cp -R "$APP" "/Applications/${APP_NAME}.app"
	echo "==> installed: /Applications/${APP_NAME}.app"
fi
