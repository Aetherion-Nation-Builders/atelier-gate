# Atelier Gate app icon

The app icon is the **Atelier Gate mark**: the hidden sun behind a gate, glowing through its arched
doorway, in Atelier's three colours (volcanic black `#141210`, ivory `#efe6d2`, ochre `#d9a441`).
It is original work for Atelier (MIT OR Apache-2.0, see [`LICENSE.txt`](LICENSE.txt)).
[`artcraft-launcher.svg`](artcraft-launcher.svg) is the master; the file names keep upstream's
machine name.

## Files

- `artcraft-launcher.svg`: the master; also the hicolor scalable icon and the mark Gate draws in
  its header and Settings.
- `artcraft-launcher-1024.png`: 1024 px render, with extra transparent margin for Apple's icon grid.
- `artcraft-launcher.icns`: macOS bundle icon (`CFBundleIconFile`).
- `artcraft-launcher.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/artcraft-launcher/build.rs` and used by the MSI.
- `hicolor/<size>/apps/ai.storyteller.artcraft-launcher.png` (16–512) and
  `hicolor/scalable/apps/ai.storyteller.artcraft-launcher.svg`: Linux icon theme.
- `tray-template-44.png`: the menu-bar icon on macOS (a template image, 22 pt @2x: the mark's
  shapes in black, which macOS tints), and `tray-64.png`: the tray icon on Windows, Linux and
  FreeBSD (the full-colour mark).

## Regenerate

Change the master, then run `packaging/icons.sh` on any OS. It needs `resvg` and python3 with
Pillow; the `.ico` and `.icns` are packed by `cargo xtask ico` / `cargo xtask icns`.
