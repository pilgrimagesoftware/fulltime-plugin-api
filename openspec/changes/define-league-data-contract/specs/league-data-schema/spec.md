## ADDED Requirements

### Requirement: Canonical Competition and Team Model
The schema SHALL define a source-agnostic representation of a competition (league, cup, or
national-team tournament) and its participating teams, independent of any single
provider's identifiers.

#### Scenario: Two plugins describe the same team consistently
- **WHEN** the Bundesliga plugin and a national-team plugin both supply data for teams
  that play in both contexts
- **THEN** each team's canonical representation uses the same schema fields (name, short
  name, canonical ID) regardless of which plugin produced it

### Requirement: Canonical Fixture and Result Model
The schema SHALL define a single fixture/result shape covering scheduled kickoff time,
participating teams, venue, status, and score, sufficient to represent both league
fixtures and knockout/group-stage fixtures.

#### Scenario: League fixture is represented
- **WHEN** the Bundesliga plugin returns a scheduled league match
- **THEN** the fixture conforms to the canonical fixture schema with status `scheduled`
  and no score

#### Scenario: Group-stage fixture is represented
- **WHEN** a national-team competition plugin returns a group-stage match that has
  finished
- **THEN** the fixture conforms to the same canonical fixture schema with status
  `finished` and a final score, using the same fields as a league fixture

### Requirement: Canonical Standings Model
The schema SHALL define a standings/table representation supporting both single-table
league formats and group-based tournament formats using the same row shape.

#### Scenario: League table standings
- **WHEN** the Bundesliga plugin returns a standings table
- **THEN** the standings conform to the canonical schema as a single ranked table

#### Scenario: Group-stage standings
- **WHEN** a national-team competition plugin returns group-stage standings
- **THEN** the standings conform to the canonical schema as multiple named groups, each a
  ranked table using the same row shape as the league table

### Requirement: Schema Version Identification
The canonical schema SHALL carry an explicit version identifier, independent of the
data-provider interface version, that both the host and plugins reference to negotiate
compatibility.

#### Scenario: Host checks plugin compatibility against schema version
- **WHEN** the host loads a plugin declaring a target schema version
- **THEN** the host compares that version against the schema version(s) it supports
  before allowing the plugin to be invoked

#### Scenario: Plugin targets an older but compatible schema minor version
- **WHEN** a plugin declares schema version `1.2` and the host implements schema version
  `1.3`
- **THEN** the host accepts the plugin, since a higher host minor version is a superset of
  the fields the plugin expects

#### Scenario: Plugin targets an incompatible major schema version
- **WHEN** a plugin declares schema version `2.0` and the host implements only schema
  version `1.x`
- **THEN** the host refuses to load the plugin and reports a version-incompatibility error
