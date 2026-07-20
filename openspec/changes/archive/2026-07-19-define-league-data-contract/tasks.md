## 1. Crate Setup

- [x] 1.1 Scaffold the `fulltime-plugin-api` Rust crate (Cargo.toml, lib.rs, license, CI)
- [x] 1.2 Pin `wit-bindgen` and `wasmtime`-compatible tooling versions, documented in the
  crate's changelog

## 2. Canonical Schema

- [x] 2.1 Define the canonical competition and team types
- [x] 2.2 Define the canonical fixture/result type covering both league and
  knockout/group-stage matches
- [x] 2.3 Define the canonical standings type supporting single-table and group-based
  formats with a shared row shape
- [x] 2.4 Add the schema version identifier and document the major/minor compatibility
  policy
- [x] 2.5 Validate the schema against a real Bundesliga data shape (blocked on `Plugins/Bundesliga`
  reaching a working prototype; validate against `Libs/openligadb`'s existing response
  shapes in the meantime)

## 3. Data-Provider WIT Interface

- [x] 3.1 Author the WIT package: list-competitions, fetch-fixtures, fetch-results,
  fetch-standings, fetch-metadata
- [x] 3.2 Define structured error variants: network-failure, rate-limited,
  schema-mapping-failure
- [x] 3.3 Add the interface version identifier, independent of the schema version, and
  document the major/minor compatibility policy
- [x] 3.4 Generate and publish Rust bindings via `wit-bindgen`

## 4. Plugin Manifest Format

- [x] 4.1 Define the manifest schema (plugin ID, version, target schema version, target
  interface version, declared network hosts)
- [x] 4.2 Implement manifest parsing with field-presence and format validation
- [x] 4.3 Add structured parse errors identifying the invalid field, with no network or
  host-runtime side effects

## 5. Publishing

- [x] 5.1 Write crate-level documentation covering the schema, WIT interface, manifest
  format, and versioning policy for plugin authors
- [x] 5.2 Cut an initial versioned release for `Apps/rust` and `Plugins/Bundesliga` to
  depend on
