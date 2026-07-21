
## [0.2.0] - 2026-07-21


### Added

- Add name and localized [names] display-name fields


### Documentation

- Drop spurious "Fixed: Conflicts" changelog entry

- Document squash-merge subject requirement in RELEASING.md

- Fix wrong export! form and stale 0.2 version references


### Fixed

- Conflicts

- Add explicit permissions to CI/release caller workflows



## [Unreleased]

### Added

- Add a required `name` field to `Manifest` for a plugin's human-readable display name,
  distinct from `id` (hosts must not derive a display name from `id`)
- Add an optional `[names]` table for locale-keyed display names, and
  `Manifest::localized_name` to look one up with fallback to `name`

## [0.1.1] - 2026-07-20

### Added

- Add the `host` WIT interface (`fetch`) and wire it into `world plugin` as an import,
  so a plugin component can no longer instantiate without a host that supplies network
  access **[BREAKING]**
- Re-export the `Guest` trait and `export!` macro for the `data-provider` interface, and
  add a `host_fetch` wrapper around the generated `fetch` import, so a downstream plugin
  can implement and export the world using this crate's own canonical types
- Bump `INTERFACE_VERSION` to `2.0` **[BREAKING]**

### Documentation

- Add badges, CONTRIBUTING, CODE_OF_CONDUCT, and RELEASING
- Document implementing/exporting the `Guest` trait and using `host_fetch` in
  `docs/plugin-authoring.md`

## [0.1.0] - 2026-07-20

### Added

- Canonical league-data schema, data-provider WIT interface, and manifest format

### Documentation

- Propose define-league-data-contract change
- Add security policy
