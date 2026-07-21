## ADDED Requirements

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
