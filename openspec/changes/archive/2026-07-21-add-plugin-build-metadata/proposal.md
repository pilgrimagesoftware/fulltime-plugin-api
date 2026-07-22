## Why

The plugin manifest currently has no field for who built a plugin or when. `fulltime-core`'s
Plugins management screen (`openspec/changes/plugin-host-runtime` in `Apps/rust`) lists each
plugin's `id` and `version` only, because that's all the manifest carries — there's nowhere to
show a developer/publisher name or a build timestamp to help a user tell plugins apart or judge
how current one is.

## What Changes

- Add two optional manifest fields: `developer` (a display name/identifier for the plugin's
  author or publisher) and `build_date` (an RFC 3339 timestamp for when the plugin was built).
  Optional, not required, so existing manifests (e.g. `Plugins/Bundesliga`'s) keep parsing
  without changes — an additive, minor-version manifest schema change under this crate's own
  versioning policy (see `RELEASING.md`/`src/version.rs`'s doc comments).
- `Manifest::parse` accepts and exposes both fields when present, and treats their absence as
  `None` rather than a parse error. Neither field affects host/plugin compatibility checks — both
  are display-only metadata, unlike `schema_version`/`interface_version`.
- `build_date` is documented as RFC 3339 (matching the existing `Fixture.kickoff` convention in
  `wit/data-provider.wit`) but is not parsed/validated by this crate — same treatment as
  `kickoff`, which this crate also never validates. A non-empty check only, matching
  `network_hosts` entries.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `plugin-manifest-format`: the manifest schema gains two optional fields, `developer` and
  `build_date`, each exposed on the parsed `Manifest` and validated at parse time when present.

## Impact

- **`src/manifest.rs`**: `Manifest` struct gains `developer: Option<String>` and
  `build_date: Option<String>` (or a parsed timestamp type — see `design.md`), `RawManifest`
  gains the corresponding optional fields, and `Manifest::parse` validates `build_date`'s format
  when present.
- **Downstream plugins** (`Plugins/Bundesliga`, future plugins): unaffected unless they choose to
  add the new fields to their own `manifest.toml`.
- **`Apps/rust`'s plugin management UI** (`openspec/changes/plugin-host-runtime`, a separate,
  already-implemented change in that repo): a follow-up change there would surface these fields
  in the Plugins screen once this manifest change ships — out of scope here.
