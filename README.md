# plugin-api

[![Crates.io](https://img.shields.io/crates/v/fulltime-plugin-api.svg)](https://crates.io/crates/fulltime-plugin-api)
[![docs.rs](https://img.shields.io/docsrs/fulltime-plugin-api)](https://docs.rs/fulltime-plugin-api)
[![CI](https://github.com/pilgrimagesoftware/fulltime-plugin-api/actions/workflows/ci.yaml/badge.svg)](https://github.com/pilgrimagesoftware/fulltime-plugin-api/actions/workflows/ci.yaml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Canonical league data schema and WIT plugin interface shared by the FullTime plugin host (`Apps/rust`) and data-provider plugins (`Plugins/Bundesliga`, and future league plugins).

Neither the host nor any plugin owns this contract - it is versioned and published independently so plugins and the host can evolve without a lockstep release.

## What's in this crate

- **Canonical `league-data-schema`** (`Competition`, `Team`, `Fixture`, `Standings`, ...): the source-agnostic data shape every plugin maps its provider's response into, and every host UI consumes.
  Covers single-table league formats and group-based tournament formats with the same types.
- **`data-provider` WIT interface** (`wit/data-provider.wit`): the contract a plugin implements - `list-competitions`, `fetch-fixtures`, `fetch-results`, `fetch-standings`, `fetch-metadata` - plus structured error variants for network failure, rate limiting, and schema-mapping failure.
  Rust bindings are generated from this file via `wit-bindgen`, not hand-written, so the WIT source is the single source of truth.
- **Plugin manifest format** (`Manifest`): the static TOML file every plugin ships declaring its ID, release version, targeted schema/interface versions, and required network hosts.
  This crate validates structure and field format only - network reachability and capability enforcement belong to the host runtime (`Apps/rust`).
- **`host` interface and `Guest`/`export!` bindings**: `world plugin` imports `host.fetch` - a plugin has no direct network access and must call this crate's `host_fetch` wrapper for every upstream request. This crate also re-exports the generated `Guest` trait and `export!` macro so a downstream plugin implements and exports the world using this crate's own canonical types, rather than regenerating an incompatible copy from a vendored WIT file.

## Versioning

The schema and the interface each carry an independent `major.minor` [`Version`] identifier (`SCHEMA_VERSION`, `INTERFACE_VERSION`), because a schema field addition and an interface function signature change are unrelated concerns and shouldn't force a shared breaking version.

Compatibility is major-version equality, minor-version-or-lower on the plugin side: a plugin declaring `1.2` loads against a host on `1.3`+ (the host is a superset) but not `1.1` (missing fields) or `2.x` (major mismatch).
See [`Version::accepts`].

## Usage

```toml
[dependencies]
fulltime-plugin-api = "0.1"
```

```rust
use fulltime_plugin_api::{Manifest, SCHEMA_VERSION};

let manifest = Manifest::parse(include_str!("../tests/fixtures/manifest.toml"))?;
assert!(SCHEMA_VERSION.accepts(manifest.schema_version));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Building a plugin

See [`docs/plugin-authoring.md`](docs/plugin-authoring.md) for a walkthrough of the WIT interface, manifest format, and versioning policy.

## Change log

[CHANGELOG](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and commit conventions, and [RELEASING.md](RELEASING.md) for how versions get cut.
This project follows the [Code of Conduct](CODE_OF_CONDUCT.md).

## Security

See [SECURITY.md](SECURITY.md) to report a vulnerability.

## License

Licensed under:

* MIT license ([LICENSE.md](LICENSE.md) or <https://opensource.org/licenses/MIT>)

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above.
