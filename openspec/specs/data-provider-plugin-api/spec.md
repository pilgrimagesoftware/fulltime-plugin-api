### Requirement: Standard Data-Provider Interface
The WIT package SHALL define a common interface exposing operations to list competitions,
fetch fixtures, fetch results, fetch standings, and fetch team/competition metadata, for
every data-provider plugin to implement.

#### Scenario: Host queries a plugin's supported operations
- **WHEN** the host inspects a loaded plugin's exported interface
- **THEN** the plugin reports which of the standard data-provider operations it implements

#### Scenario: Host calls an operation the plugin implements
- **WHEN** the host invokes `fetch-fixtures` on a plugin that implements it
- **THEN** the plugin returns fixture data conforming to the canonical `league-data-schema`

### Requirement: Canonical Schema Output
Every operation in the interface SHALL return data typed against the canonical
`league-data-schema`; the interface SHALL NOT define plugin-specific or provider-specific
return types.

#### Scenario: Plugin returns data in canonical schema
- **WHEN** a plugin successfully fetches fixtures from its upstream source
- **THEN** the WIT function signature constrains the return type to the canonical fixture
  schema, so the host never receives an untyped or provider-specific shape

### Requirement: Structured Error Types
The interface SHALL define structured error variants a plugin returns for upstream
failures — network failure, rate limit, and schema-mapping failure — instead of letting
failures surface as unhandled traps.

#### Scenario: Upstream source is unreachable
- **WHEN** a plugin's upstream HTTP call fails due to a network error
- **THEN** the plugin returns the `network-failure` error variant, and the host can
  distinguish it from a successful empty result

#### Scenario: Upstream source rate-limits the plugin
- **WHEN** a plugin's upstream source responds with a rate-limit error
- **THEN** the plugin returns the `rate-limited` error variant, which the host can use to
  back off and retry later

#### Scenario: Plugin cannot map upstream data to the schema
- **WHEN** a plugin receives upstream data it cannot represent in the canonical schema
- **THEN** the plugin returns the `schema-mapping-failure` error variant rather than
  partial or malformed schema data

### Requirement: Downstream Implementation Bindings
This crate SHALL expose the generated `Guest` trait and `export!` macro for the
`data-provider` interface, so a downstream plugin can implement and export the world using
this crate's own canonical types instead of regenerating an incompatible copy from a
vendored WIT file.

#### Scenario: Plugin implements the Guest trait
- **WHEN** a plugin crate depends on this crate as an ordinary Rust library
- **THEN** it can implement this crate's re-exported `Guest` trait for `data-provider`
  using this crate's own `Team`/`Fixture`/`Standings`/`Competition`/`ProviderError` types,
  with no separate WIT-derived type set of its own

#### Scenario: Plugin exports its implementation
- **WHEN** a plugin has implemented the `Guest` trait
- **THEN** it calls this crate's re-exported `export!` macro to export the implementation
  as the component's `data-provider` interface, without needing its own
  `wit_bindgen::generate!` invocation

### Requirement: Interface Versioning
The data-provider interface SHALL carry an explicit version identifier, independent of the
schema version, so the host can detect and reject plugins built against an incompatible
interface version before invoking them.

`INTERFACE_VERSION`'s major component covers both axes of compatibility: the shape of the
`data-provider` exports a plugin implements, and the set of imports (currently, `host.fetch`)
a plugin requires from the host. A change to either axis that a plugin built against an
older major version cannot satisfy is a major bump; before `host.fetch` existed, only the
export shape was covered.

#### Scenario: Plugin built against a newer interface than the host supports
- **WHEN** the host loads a plugin declaring an interface version newer (major) than any
  version the host implements
- **THEN** the host refuses to load the plugin and reports a version-incompatibility error

#### Scenario: Plugin built against an older, compatible interface version
- **WHEN** the host loads a plugin declaring an interface minor version lower than the
  host's supported version, with the same major version
- **THEN** the host loads the plugin, since the host's interface is a superset of the
  functions the plugin was built against, and the plugin requires no imports the host
  cannot supply

#### Scenario: Plugin built before the host-fetch import existed
- **WHEN** the host loads a plugin declaring `interface_version` `1.x` (built before
  `host.fetch` was added to the `plugin` world)
- **THEN** the host refuses to load the plugin as a major-version mismatch against its own
  `2.x` support, rather than attempting instantiation and failing at the component-linking
  stage with a less informative error
