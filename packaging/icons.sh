#!/usr/bin/env bash
# Regenerate every app and tray icon from assets/app-icon/artcraft-launcher.svg, the Atelier Gate
# mark (the hidden sun behind a gate; original work for Atelier, MIT OR Apache-2.0).
#
# - App icon (Dock, taskbar, Start menu, Linux menus, installers): the full-colour mark.
# - Tray: tray-template-44.png (the mark's shapes in black, a macOS menu-bar template) and
#   tray-64.png (the full-colour mark; Windows, Linux and BSD).
#
# Needs: resvg (brew install resvg / cargo install resvg) and python3 with Pillow (for the template).
# The .ico and .icns are packed by `cargo xtask ico` / `cargo xtask icns`, so this runs on any OS.
# The outputs are committed, so packaging (and the Rust build, which include_bytes!s some of them)
# never needs these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/artcraft-launcher.svg"
ID="ai.storyteller.artcraft-launcher"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }

# macOS icons get a little more transparent margin (Apple's 824/1024 icon grid) by widening the
# view box around the artwork; everything else uses the file as is.
BOX='viewBox="0 0 512 512"'
grep -q "$BOX" "$SVG" || { echo "error: expected $BOX in $SVG" >&2; exit 1; }
MAC="$TMP/mac.svg"
sed "s/$BOX/viewBox=\"-62.136 -62.136 636.272 636.272\"/" "$SVG" >"$MAC"
FLAT="$SVG"

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }

render "$MAC" 1024 "$DIR/artcraft-launcher-1024.png"

# Linux hicolor theme.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$FLAT" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$ID.svg"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$FLAT" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/artcraft-launcher.ico" "${ICO_PNGS[@]}")

# macOS .icns.
ICNS_PNGS=()
for s in 16 32 64 128 256 512 1024; do
  render "$MAC" "$s" "$TMP/icns-$s.png"
  ICNS_PNGS+=("$TMP/icns-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- icns "$DIR/artcraft-launcher.icns" "${ICNS_PNGS[@]}")

# Tray icons: the colour mark at 64 px, and the macOS template (44 px = 22 pt @2x): the mark
# without its tile, cropped to the artwork, light shapes kept as black and the dark gaps cleared.
render "$FLAT" 64 "$DIR/tray-64.png"
sed -e 's#<rect width="512" height="512" rx="112" fill="\#141210"/>##' -e 's#viewBox="0 0 512 512"#viewBox="96 56 320 404"#' "$SVG" >"$TMP/tray.svg"
resvg -w 35 -h 44 "$TMP/tray.svg" "$TMP/tray.png" </dev/null
python3 - "$TMP/tray.png" "$DIR/tray-template-44.png" <<'PY'
import sys
from PIL import Image
src = Image.open(sys.argv[1]).convert("RGBA")
out = Image.new("RGBA", (44, 44), (0, 0, 0, 0))
for y in range(src.height):
    for x in range(src.width):
        r, g, b, a = src.getpixel((x, y))
        lum = (r + g + b) / 3 / 255
        out.putpixel((x + 4, y), (0, 0, 0, int(a * max(0.0, min(1.0, (lum - 0.1) / 0.5)))))
out.save(sys.argv[2])
PY
echo "icons written to $DIR"
