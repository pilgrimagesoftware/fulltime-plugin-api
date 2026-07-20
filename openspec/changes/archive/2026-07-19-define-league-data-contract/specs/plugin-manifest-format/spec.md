## ADDED Requirements

### Requirement: Plugin Manifest Schema
This crate SHALL define the static manifest schema every plugin ships: plugin ID, version,
target schema version, target interface version, and the set of network hosts it requires
access to.

#### Scenario: Manifest declares required fields
- **WHEN** a manifest is parsed against this crate's schema
- **THEN** parsing requires plugin ID, version, target schema version, target interface
  version, and declared network hosts to be present, and fails if any is missing

#### Scenario: Manifest declares network capabilities
- **WHEN** a plugin's manifest lists the hostnames it needs to call
- **THEN** the parsed manifest exposes exactly that list, in a form the host runtime can
  use to scope the plugin's HTTP fetch capability at load time

### Requirement: Manifest Format Validation Only
This crate SHALL validate manifest structure and field presence/format; it SHALL NOT
perform host-side enforcement decisions (network reachability, capability granting,
enable/disable state) — those belong to the plugin host runtime.

#### Scenario: Malformed manifest is rejected at parse time
- **WHEN** a manifest file has an invalid version string or a malformed hostname entry
- **THEN** parsing returns a structured error identifying the invalid field, without
  attempting to contact any declared host

#### Scenario: Well-formed manifest is accepted regardless of runtime policy
- **WHEN** a manifest is structurally valid but declares a network host the host runtime
  will later reject for policy reasons
- **THEN** this crate parses the manifest successfully; the runtime enforcement decision
  happens outside this crate
