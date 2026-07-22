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

### Requirement: Plugin Build Metadata
The manifest schema SHALL support two optional display-only fields: `developer` (a
display name/identifier for the plugin's author or publisher) and `build_date` (a
timestamp, conventionally RFC 3339, for when the plugin was built). Neither field SHALL be
required, and neither SHALL affect schema/interface compatibility checks.

#### Scenario: Manifest omits both fields
- **WHEN** a manifest has no `developer` or `build_date` field
- **THEN** parsing succeeds and the parsed manifest exposes both as absent, not as an error

#### Scenario: Manifest declares a developer name
- **WHEN** a manifest includes a non-empty `developer` field
- **THEN** the parsed manifest exposes that value unchanged

#### Scenario: Manifest declares a build date
- **WHEN** a manifest includes a non-empty `build_date` field
- **THEN** the parsed manifest exposes that value unchanged, without being parsed or validated
  as a timestamp

#### Scenario: Empty developer or build_date field is rejected
- **WHEN** a manifest includes a `developer` or `build_date` field present but empty (or
  whitespace-only)
- **THEN** parsing fails with a structured error identifying the invalid field, the same way an
  empty `network_hosts` entry is rejected
