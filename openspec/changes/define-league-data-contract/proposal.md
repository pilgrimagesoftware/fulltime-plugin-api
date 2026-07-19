## Why

The FullTime plugin host (`Apps/rust`) and every data-provider plugin (starting with
`Plugins/Bundesliga`) need a shared contract neither side owns: a canonical league data
schema and a WIT interface for the host to call into plugins. Without a stable,
independently-versioned contract, the host and plugins would be forced into lockstep
releases, defeating the point of a plugin architecture (see the umbrella change,
`FullTime#1`, `openspec/changes/league-data-plugin-system`).

## What Changes

- Define the canonical `league-data-schema`: competitions, teams, fixtures, results, and
  standings, covering both single-table league formats and group-based tournament formats.
- Add an explicit schema version identifier and document the compatibility policy for
  future breaking changes.
- Define the WIT interface for the data-provider plugin API: list competitions, fetch
  fixtures, fetch results, fetch standings, fetch metadata.
- Define structured plugin error types: network failure, rate limit, schema-mapping
  failure.
- Define the interface version scheme and the host-side compatibility check plugins are
  validated against.
- Define the plugin manifest format: plugin ID, version, target interface/schema version,
  and declared network hosts.

## Capabilities

### New Capabilities

- `league-data-schema`: canonical, source-agnostic representation of competitions, teams,
  fixtures, results, and standings, versioned and shared by the host and all plugins.
- `data-provider-plugin-api`: the WIT contract a plugin implements to supply league data to
  the host, plus its structured error types and version-compatibility rules.
- `plugin-manifest-format`: the static manifest schema (identity, version, targeted
  interface/schema version, declared network hosts) the host reads before loading a plugin.

### Modified Capabilities

- (none — this is a new repo, no existing specs predate this change)

## Impact

- **This repo (`fulltime-plugin-api`)**: net-new. Publishes the schema types, WIT
  definitions, and manifest format as a versioned crate/package.
- **`Apps/rust`**: the plugin host runtime (separate child change) will depend on this
  contract to validate and load plugins, and will map plugin output to these schema types
  for the UI.
- **`Plugins/Bundesliga`**: the reference plugin (separate child change) will implement this
  WIT interface and ship a manifest conforming to this format.
- Out of scope: the WASM host runtime itself, the Bundesliga plugin implementation, and any
  app UI — those are tracked as separate child changes against `Apps/rust` and
  `Plugins/Bundesliga`.
