//! Atelier: the twelve apps Atelier Gate knows about (each built on a Craft App; see docs/brand.md).
//!
//! The list is data: `crates/core/catalog.toml`, compiled in as [`SUITE`] by `build.rs`. Upstream
//! can replace it at startup with a signed catalog from its website; Gate doesn't
//! ([`crate::hosted_catalog::ENABLED`]).
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// Who makes an app. Every Atelier app is the Nation's own ([`AppKind::Official`]); upstream also
/// lists community apps, which Gate leaves out until the Nation forks them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppKind {
    Official,
    Community,
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`Pigment`).
    pub name: &'static str,
    /// The full product name (`Atelier Pigment`): what the app's Windows installer registers (the
    /// Apps & features entry and the `Program Files` folder).
    pub product: &'static str,
    /// Upstream's product name (`PhotoCraft`): the macOS bundle name of releases made before the
    /// app's Atelier installer, and the `Info.plist` version key (`PhotoCraftVersion`), which
    /// keeps it. Atelier installers name the bundle after [`Self::product`]
    /// ([`Self::mac_bundle_names`]).
    pub upstream_name: &'static str,
    /// The `.app` bundle stem on macOS when it differs from `name` (`Atelier Pigment` for
    /// `Pigment`): what [`Self::mac_bundle_name`] reports, and the first of
    /// [`Self::mac_bundle_names`].
    pub mac_bundle_name: Option<&'static str>,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (PdfCraft's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
    pub kind: AppKind,
    /// Who publishes it, as the card's byline shows it (`Aetherion Nation`).
    pub publisher: &'static str,
    /// The publisher its Windows installer names, when it isn't the Nation's.
    pub msi_publisher: Option<&'static str>,
    /// The bundle id when it isn't `ai.storyteller.<id>`.
    pub bundle: Option<&'static str>,
    /// Older bundle ids its Flatpak shipped under.
    pub former_bundles: &'static [&'static str],
    /// Its website, when it isn't its page on the Nation's site.
    pub site: Option<&'static str>,
    /// The Apple Developer team whose Developer ID signature its macOS build must carry. `None`
    /// for every Atelier app until the Nation signs its releases: Gate then opens a copy that's
    /// there and links to the releases, but installs nothing on macOS.
    pub mac_team: Option<&'static str>,
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        self.bundle.map_or_else(|| format!("ai.storyteller.{}", self.id), str::to_string)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The primary `.app` bundle stem on macOS (`Atelier Pigment`).
    pub fn mac_bundle_name(&self) -> &'static str {
        self.mac_bundle_name.unwrap_or(self.name)
    }

    /// The names its macOS app bundle may have, newest first, without `.app`: the Atelier name
    /// (`Atelier Pigment`), then upstream's (`PhotoCraft`), which earlier releases used.
    pub fn mac_bundle_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        let primary = self.mac_bundle_name.unwrap_or(self.product);
        std::iter::once(primary)
            .chain(std::iter::once(self.product).filter(move |&n| n != primary))
            .chain(std::iter::once(self.upstream_name).filter(move |&n| n != primary && n != self.product))
    }

    /// The app's page on the Nation's site.
    pub fn website(&self) -> String {
        self.site.map_or_else(|| format!("{WEBSITE_URL}/atelier/{}", self.name.to_lowercase()), str::to_string)
    }

    /// Whether the launcher installs this app from a `kind` package.
    pub fn installs(&self, kind: crate::PackageKind) -> bool {
        kind != crate::PackageKind::Dmg || self.mac_team.is_some() || INSTALLS_UNSIGNED_MAC_BUILDS
    }

    /// Whether this app runs on the given operating system (every Atelier app runs everywhere;
    /// upstream's ArtCraft Studio skipped FreeBSD).
    pub fn runs_on(&self, os: crate::Os) -> bool {
        match os {
            crate::Os::FreeBsd => self.id != "artcraft",
            _ => true,
        }
    }

    /// Bundle ids this app may be installed under, current first.
    pub fn bundle_ids(&self) -> impl Iterator<Item = String> + '_ {
        std::iter::once(self.bundle_id()).chain(self.former_bundles.iter().map(ToString::to_string))
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first: the product name
    /// (`Atelier Seal`) the Atelier installers register, then any older ones.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.product).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        crate::hosted_catalog::apps().iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    [name, product, upstream_name]: [&'static str; 3],
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp {
        id,
        name,
        product,
        upstream_name,
        tagline,
        color,
        ink_light,
        ink_dark,
        repo,
        former_ids: &[],
        former_names: &[],
        mac_bundle_name: None,
        kind: AppKind::Official,
        publisher: "Aetherion Nation",
        msi_publisher: None,
        bundle: None,
        former_bundles: &[],
        site: None,
        // The Nation's releases are unsigned (see `mac_team`).
        mac_team: None,
    }
}

// Atelier, in display order (upstream's order, which follows its website): from catalog.toml.
include!(concat!(env!("OUT_DIR"), "/catalog.rs"));

/// Atelier: the Nation's macOS releases carry no Developer ID signature yet, so Gate installs a
/// DMG with no signing team ([`CraftApp::mac_team`] `None`) after checking the download's
/// checksum and the bundle's id and version, as Atelier Gate always has. Upstream refuses such a
/// build; turn this off (and record each app's team) once the Nation signs its releases.
pub const INSTALLS_UNSIGNED_MAC_BUILDS: bool = true;

/// Upstream's Apple Developer team (Learning Machines LLC), which signs the official Craft Apps.
/// No Atelier app carries it (their `mac_team` is `None`); upstream's hosted-catalog checks name it.
pub const ARTCRAFT_TEAM_ID: &str = "DJ6XS33FX8";

/// The Nation's website.
pub const WEBSITE_URL: &str = "https://www.aetherionnnation.com";
/// The GitHub account every Atelier app's repository lives under: the only one Gate installs
/// from, whatever a release feed says.
pub const GITHUB_OWNER: &str = "Aetherion-Nation-Builders";
/// Atelier on GitHub: every Atelier app's source.
pub const GITHUB_URL: &str = "https://github.com/Aetherion-Nation-Builders";
/// The Nation's social accounts: (service, URL), shown as small icons. None yet; a service needs
/// an icon in `ui-egui`'s `icons::social` (X, Instagram, LinkedIn, YouTube, TikTok are there).
pub const SOCIAL: [(&str, &str); 0] = [];

/// Joining the Nation (the community link that was ArtCraft's Discord upstream).
pub const NATION_URL: &str = "https://www.aetherionnnation.com";
/// Upstream's names for its community link, which it opens in the Discord app when one is
/// installed. Both point at the Nation, so that code path opens the Nation's site.
pub const DISCORD_URL: &str = NATION_URL;
/// See [`DISCORD_URL`]: no app deep link, the Nation's site.
pub const DISCORD_APP_URL: &str = NATION_URL;

/// Atelier Gate itself, described like an app so it can update itself with the same machinery
/// (asset names `artcraft-launcher-<ver>-…`, bundle id `ai.storyteller.artcraft-launcher`,
/// `ArtCraft Launcher.app`: machine names kept from upstream). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "artcraft-launcher",
    ["Atelier Gate", "Atelier Gate", "ArtCraft Launcher"],
    "Installs and updates the Atelier apps",
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "Aetherion-Nation-Builders/atelier-gate",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "Aetherion-Nation-Builders/atelier-gate";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "artcraft-launcher";

/// An Atelier app's 128 px icon (PNG), in `assets/app-icons` (each Atelier app's own): the UI
/// draws it, and the Linux install puts it in the desktop menu.
pub fn app_icon_png(id: &str) -> Option<&'static [u8]> {
    if let Some(icon) = crate::hosted_catalog::icon(id) {
        return Some(icon);
    }
    Some(match id {
        "photocraft" => include_bytes!("../../../assets/app-icons/photocraft.png"),
        "vectorcraft" => include_bytes!("../../../assets/app-icons/vectorcraft.png"),
        "filmcraft" => include_bytes!("../../../assets/app-icons/filmcraft.png"),
        "lightcraft" => include_bytes!("../../../assets/app-icons/lightcraft.png"),
        "pdfcraft" => include_bytes!("../../../assets/app-icons/pdfcraft.png"),
        "effectcraft" => include_bytes!("../../../assets/app-icons/effectcraft.png"),
        "designcraft" => include_bytes!("../../../assets/app-icons/designcraft.png"),
        "wordcraft" => include_bytes!("../../../assets/app-icons/wordcraft.png"),
        "deckcraft" => include_bytes!("../../../assets/app-icons/deckcraft.png"),
        "gridcraft" => include_bytes!("../../../assets/app-icons/gridcraft.png"),
        "cadcraft" => include_bytes!("../../../assets/app-icons/cadcraft.png"),
        "soundcraft" => include_bytes!("../../../assets/app-icons/soundcraft.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_app_has_its_icon() {
        for app in &SUITE {
            let png = app_icon_png(app.id).unwrap_or_else(|| panic!("no icon for {}", app.id));
            assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "{} icon isn't a PNG", app.id);
        }
        assert!(app_icon_png("notacraft").is_none());
    }

    #[test]
    fn every_app_is_the_nations_and_unsigned_on_macos_for_now() {
        for a in &SUITE {
            assert_eq!((a.kind, a.publisher, a.mac_team), (AppKind::Official, "Aetherion Nation", None), "{}", a.id);
            assert_eq!(a.bundle_id(), format!("ai.storyteller.{}", a.id));
            assert!(a.installs(crate::PackageKind::Dmg) && a.installs(crate::PackageKind::Msi), "{}", a.id);
        }
    }

    #[test]
    fn suite_has_twelve_apps_with_unique_ids() {
        assert_eq!(SUITE.len(), 12);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("Aetherion-Nation-Builders/atelier-{}", a.name.to_lowercase()));
            assert!(a.id.ends_with("craft"), "{}: machine names stay upstream's", a.id);
            assert_eq!(a.product, format!("Atelier {}", a.name));
            assert_eq!(a.upstream_name.to_lowercase(), a.id, "macOS bundles keep upstream's name");
            assert_eq!(a.mac_bundle_name, Some(a.product));
            assert_eq!(a.website(), format!("{WEBSITE_URL}/atelier/{}", a.name.to_lowercase()));
        }
        assert_eq!(LAUNCHER.repo, LAUNCHER_REPO);
    }

    #[test]
    fn macos_bundles_are_found_under_the_atelier_name_first() {
        let pigment = CraftApp::by_id("photocraft").unwrap();
        assert_eq!(pigment.mac_bundle_name(), "Atelier Pigment");
        assert_eq!(pigment.mac_bundle_names().collect::<Vec<_>>(), ["Atelier Pigment", "PhotoCraft"]);
        assert_eq!(LAUNCHER.mac_bundle_names().collect::<Vec<_>>(), ["Atelier Gate", "ArtCraft Launcher"]);
        for os in [crate::Os::FreeBsd, crate::Os::Linux, crate::Os::MacOs, crate::Os::Windows] {
            assert!(SUITE.iter().all(|a| a.runs_on(os)));
        }
    }

    #[test]
    fn pdfcraft_also_answers_to_printcraft() {
        let pdf = CraftApp::by_id("pdfcraft").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["pdfcraft", "printcraft"]);
        assert_eq!(pdf.names().collect::<Vec<_>>(), ["Atelier Seal", "PrintCraft"]);
        assert_eq!(pdf.name, "Seal");
        assert_eq!(pdf.website(), "https://www.aetherionnnation.com/atelier/seal");
        assert_eq!(CraftApp::by_id("wordcraft").unwrap().website(), "https://www.aetherionnnation.com/atelier/scribe");
    }
}
