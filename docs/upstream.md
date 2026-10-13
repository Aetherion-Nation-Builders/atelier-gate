# Pulling in upstream work

Upstream is [storytold/craft-launcher](https://github.com/storytold/craft-launcher) (ArtCraft
Launcher). We merge its `main` into ours; our changes stay a thin layer on top (`docs/brand.md`).

```bash
scripts/sync-upstream.sh            # fetch upstream, merge into a branch, run the checks
```

The script adds the `upstream` remote if it is missing, creates `sync/upstream-<date>` from
`main`, merges `upstream/main`, and leaves the checks to you (`cargo xtask ci`). Open a pull
request from that branch; `main` is protected and is only changed through pull requests.

Conflicts come from the files the brand layer touches. `NOTICE` keeps our version automatically
(`.gitattributes` marks it `merge=ours`; the script enables that merge driver). `README.md` is
our header on top of upstream's feature sections, so upstream README edits merge in below the
line; if the header conflicts, keep ours. For anything else, keep upstream's logic and re-apply
the rename. Three things need a look on every sync, because upstream changes them often:

- `crates/core/src/catalog.rs`: a new app upstream needs an Atelier app (name, repo, icon) before
  Gate lists it; keep our names, repos and links.
- `crates/ui-egui/src/i18n/*.tsv`: new rows that mention ArtCraft, Discord or the Craft App Suite.
- `assets/app-icons/`: keep ours (each Atelier app's own icon), never upstream's.

CI here is upstream's: `ci.yml` (tests, clippy and the wasm check) on every pull request, and
packaging lint on pull requests that touch packaging, workflows or the app icon. Turned off in
this repository, on purpose (they cost Actions minutes and build things we do not ship yet):
`release.yml`, `freebsd.yml` and `windows-arm64.yml`. Turn one on from the Actions tab when it is
wanted.
