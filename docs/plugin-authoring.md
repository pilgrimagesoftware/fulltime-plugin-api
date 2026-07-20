# Authoring a data-provider plugin

A data-provider plugin is a WASM component that implements the `data-provider` interface
defined in [`wit/data-provider.wit`](../wit/data-provider.wit) and ships a manifest
declaring its identity and required network access. This crate (`fulltime-plugin-api`) is
the only dependency required to build one: it provides the canonical schema types, the
generated interface bindings, and the manifest parser.

Neither the host (`Apps/rust`) nor any plugin's implementation lives in this repo — this
repo only defines the contract. The reference implementation is `Plugins/Bundesliga`,
wrapping `Libs/openligadb`.

## The contract

### 1. Canonical schema

Every value a plugin returns is typed against the canonical schema, not a provider-specific
shape:

- `Competition` — a league, cup, or tournament.
- `Team` — a participant, identified consistently across every plugin.
- `Fixture` — a scheduled or completed match. The optional `group` field distinguishes
  group-stage fixtures (e.g. `"Group A"`) from single-table league fixtures (`None`).
- `Standings` — one or more `StandingsGroup`s, each a ranked list of `StandingsRow`. A
  single-table league format is one unnamed group; a group-stage tournament is several
  named groups sharing the same row shape.

See the type definitions in [`wit/data-provider.wit`](../wit/data-provider.wit) (`interface
types`) for the authoritative field list — the Rust types in this crate are generated
directly from it, so the two never drift.

### 2. The `data-provider` interface

Implement the five operations in `interface data-provider`:

| Function | Returns |
|---|---|
| `list-competitions` | `list<competition>` |
| `fetch-fixtures` | `list<fixture>` for a competition (scheduled/live) |
| `fetch-results` | `list<fixture>` for a competition (finished) |
| `fetch-standings` | `standings` for a competition |
| `fetch-metadata` | `competition` |

Every function returns a `result<T, provider-error>`. Use the structured error variants
instead of letting an upstream failure surface as an unhandled trap:

- `network-failure` — the upstream HTTP call failed at the network layer.
- `rate-limited` — the upstream source responded with a rate limit, optionally carrying
  `retry-after-seconds`.
- `schema-mapping-failure` — the upstream response can't be represented in the canonical
  schema.

### 3. The manifest

Every plugin ships a TOML manifest, parsed at load time by [`Manifest::parse`]:

```toml
id = "bundesliga"
version = "0.1.0"
schema_version = "1.0"
interface_version = "1.0"
network_hosts = ["api.openligadb.de"]
```

- `id` — unique among plugins the host loads.
- `version` — the plugin's own release version, unrelated to the contract versions below.
- `schema_version` / `interface_version` — the `major.minor` contract versions this plugin
  was built against (see [`SCHEMA_VERSION`] and [`INTERFACE_VERSION`]).
- `network_hosts` — every hostname the plugin needs to call. The host runtime scopes the
  plugin's HTTP fetch capability to exactly this list; a plugin has no network access to
  anything not declared here. This crate validates the field is present and well-formed,
  not that the hosts are reachable — that enforcement lives in the host runtime.

## Versioning

The schema and the interface version independently, each as a `major.minor` pair, because a
schema field addition and an interface signature change are unrelated concerns. A host
accepts a plugin when, for both versions, the major matches and the host's minor is equal
to or greater than the plugin's declared minor — the host being a superset of what the
plugin expects. See [`Version::accepts`].

When either version changes:

- **Minor bump**: additive only (new optional field, new function that doesn't change
  existing signatures). Existing plugins keep working against a newer host.
- **Major bump**: breaking (removed/renamed field, changed function signature). Plugins
  must be rebuilt and republish their manifest's target version.

## Getting started

1. Add this crate as a dependency:
   ```toml
   [dependencies]
   fulltime-plugin-api = "0.1"
   ```
2. Implement the `data-provider` world's exported interface against your upstream data
   source, mapping its response shape into the canonical schema types.
3. Write your `manifest.toml` declaring the network hosts you call.
4. Build to a WASM component target and load it against the host runtime in `Apps/rust`.

[`Manifest::parse`]: https://docs.rs/fulltime-plugin-api/latest/fulltime_plugin_api/struct.Manifest.html#method.parse
[`SCHEMA_VERSION`]: https://docs.rs/fulltime-plugin-api/latest/fulltime_plugin_api/constant.SCHEMA_VERSION.html
[`INTERFACE_VERSION`]: https://docs.rs/fulltime-plugin-api/latest/fulltime_plugin_api/constant.INTERFACE_VERSION.html
[`Version::accepts`]: https://docs.rs/fulltime-plugin-api/latest/fulltime_plugin_api/struct.Version.html#method.accepts
