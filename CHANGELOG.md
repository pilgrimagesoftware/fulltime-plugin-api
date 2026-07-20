

### Added

- Canonical league-data schema, data-provider WIT interface, and manifest format


### Documentation

- Propose define-league-data-contract change

- Add security policy


# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added

- Canonical `league-data-schema` types: competitions, teams, fixtures/results, and standings,
  covering both single-table and group-based competition formats.
- `data-provider-plugin-api` WIT interface (`wit/data-provider.wit`) for plugins to implement,
  plus generated Rust bindings.
- `plugin-manifest-format`: plugin manifest schema and parser.
- Independent schema and interface version identifiers, each with major/minor
  consumer-compatibility semantics.

### Tooling

- Pinned `wit-bindgen` to `0.59.0` (MSRV `1.85.0`), matched by `wasmtime` `46.0.1` on the host
  side (`Apps/rust`). Bump both together when either changes.
