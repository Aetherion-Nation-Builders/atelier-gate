# The brand layer

Atelier Gate is ArtCraft Launcher with a different name on the door, pointed at the Nation's own
apps. We keep the change small on purpose: upstream lands changes often, and every file we touch
is a file that can conflict when we pull them in.

**Renamed (what a person sees):** the window title, header (the Gate mark and "Atelier", a "Gate"
tag, "Aetherion Nation"), Settings, the tray / menu-bar menu and tooltip, notifications, the
"start when I log in" entry, messages, the Russian and Chinese catalogs, the app names in the
catalog (Pigment, Quill, Reel, Lens, Seal, Prism, Press, Scribe, Podium, Ledger, Blueprint,
Frequency) and their icons, every link (the Nation's site, each app's page at
`aetherionnnation.com/atelier/<app>`, the `Aetherion-Nation-Builders` GitHub org; the Discord link
is now "Nation"), Windows file properties, installer and package names, vendor, links and setup
wizard text and art, the browser page, the app and tray icons, the README screenshots, the
User-Agent (`AtelierGate/<version>`), and the settings folder on macOS and Windows
(`Atelier Gate`). The social icons are gone until the Nation lists its accounts in
`catalog::SOCIAL`.

**Changed in what Gate does (so the brand holds):**

- Each app's releases come from the Nation's repo, `Aetherion-Nation-Builders/atelier-<app>`, and
  Gate updates itself from `atelier-gate`. Upstream installs from `storytold/<app>` and would
  replace Gate with ArtCraft Launcher on its first self-update.
- No release feed: upstream asks ArtCraft's service (`api.storyteller.ai`) first, which describes
  the storytold repos and carries ArtCraft's announcements. `feed::FEED_URL` is unset, so Gate asks
  GitHub directly (hourly per app once everything has loaded). `ARTCRAFT_LAUNCHER_FEED_URL` still
  points Gate at a feed if the Nation runs one.
- A repo that can't be read (private, or with no published release) shows its app as "Coming
  soon". Gate installs nothing until the Atelier repos publish releases it can read.

**Not renamed (machine names):** crate and binary names (`artcraft-launcher*`), the
`ai.storyteller.artcraft-launcher` app id (macOS bundle, Flatpak, Linux desktop file, Windows
AppUserModelID, LaunchAgent), release asset names (`artcraft-launcher-<ver>-…`), the apps' machine
ids (`photocraft`… in the catalog and `assets/app-icons/`), the macOS bundle and executable name
(`ArtCraft Launcher.app`, which self-update relies on), the Windows registry key
(`HKCU\Software\Learning Machines LLC\ArtCraft Launcher`) and MSI upgrade code, the portable
data folder (`ArtCraftLauncherData`), the eframe app name, environment variables
(`ARTCRAFT_LAUNCHER_*`), the Linux settings folder, and `AGENTS.md` (upstream's agent guide; this
file overrides it where they differ, e.g. on where downloads come from).

## Where the name lives

| Place | File |
|---|---|
| Apps, repos, links, the feed | `crates/core/src/catalog.rs`, `crates/core/src/feed.rs` (`FEED_URL`), `crates/native/src/backend.rs` (`feed_url`) |
| Header, footer, cards, Settings | `crates/ui-egui/src/views.rs`, `dialogs.rs`, `widgets.rs`, `icons.rs` (`gate_mark`) |
| Translations | `crates/ui-egui/src/i18n/ru.tsv`, `zh-CN.tsv` |
| Tray, window, notifications | `apps/artcraft-launcher/src/tray.rs`, `main.rs` |
| Login item, settings folder, User-Agent, messages | `crates/native/src/login.rs`, `paths.rs`, `http.rs`, `backend.rs`, `self_update.rs` |
| Status and source labels | `crates/core/src/status.rs`, `protocol.rs` |
| Windows file properties | `apps/artcraft-launcher/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright; bundle and executable names unchanged) |
| Windows installer | `packaging/windows/artcraft-launcher.wxs`, `.en-us.wxl`, `package.ps1` (wizard art), `sign.ps1`, with its check in `.github/workflows/windows-arm64.yml` |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.artcraft-launcher.*`, `nfpm.yaml` |
| Browser build | `apps/artcraft-launcher-web/index.html`, `src/web.rs` |
| Icons | `assets/app-icon/` (ours: `artcraft-launcher.svg` is the master, `packaging/icons.sh` renders the rest), `assets/app-icons/` (each Atelier app's icon) |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |

For the packaging pass (real installers): the macOS `.app` bundle, executable and DMG names
(`packaging/macos/package.sh`, `verify.sh`, and `self_update.rs` with them), the package scripts'
messages and file names, the Linux package maintainer, the AppStream brand colours, and the
release titles in `release.yml`.
