## Context

`Apps/rust` (the plugin host) and `Plugins/Bundesliga` (the reference plugin, wrapping
`Libs/openligadb`) both need to agree on a data shape and a call interface without either
depending on the other's internals. The umbrella change (`FullTime#1`,
`openspec/changes/league-data-plugin-system`) picked `wasmtime` + the Component Model
(WIT) as the plugin runtime; this repo owns the WIT source of truth plus the Rust types
generated from it.

## Goals / Non-Goals

**Goals:**
- Give the host and every plugin a single, versioned Rust crate (and WIT package) to
  depend on for the data shape and call interface.
- Make schema/interface version compatibility checkable by the host before it invokes a
  plugin, without executing plugin code.
- Cover both single-table league formats (Bundesliga) and group-based tournament formats
  (national-team qualifiers) with the same schema, per the umbrella design's risk note that
  the schema must fit both from the outset.

**Non-Goals:**
- Implementing the WASM host runtime (loading, sandboxing, fault isolation) — that's a
  separate child change in `Apps/rust`.
- Implementing any plugin against this interface — that's `Plugins/Bundesliga`'s child
  change.
- Plugin discovery, install/enable/disable state, or update tracking — that's the
  `plugin-manifest-registry` capability, owned by the host runtime change, not this repo.
  This repo only defines the manifest *format* the registry reads.

## Decisions

**WIT interface lives in this repo as the single source of truth; Rust bindings are
generated via `wit-bindgen`, not hand-written.**
Rationale: matches the umbrella decision to use the Component Model over a hand-rolled
ABI. Hand-written bindings on both host and plugin sides is exactly the drift risk WIT
exists to prevent.

**Canonical schema models fixtures/standings around the union of single-table and
group-based formats from day one**, using an optional `group` field on standings rows and
fixtures rather than two parallel type hierarchies.
Rationale: per the umbrella design's stated risk, retrofitting group support later would
break the schema version for every existing plugin. Modeling one shape now that
degrades to a single implicit group for league formats avoids a second breaking version
before any second plugin ships.

**Schema version and interface version are tracked independently**, each as a semver-like
integer pair (major.minor), with the manifest declaring which of each it targets.
Rationale: the schema (data shape) and the interface (function signatures) can each change
independently — a new field on `Fixture` doesn't necessarily require a new WIT function
signature, and vice versa. Coupling them into one version number would force unrelated
breaking changes together.

**Compatibility check is major-version equality, minor-version-or-lower on the plugin
side.**
A plugin targeting schema `1.2` loads against host schema `1.3`+ (host is a superset) but
not `1.1` (host is missing fields the plugin expects) and never `2.x` (major mismatch).
Rationale: standard semver consumer-compatibility reasoning, applied to both the schema and
the interface independently.

## Risks / Trade-offs

- [Schema won't fit a real group-stage competition's edge cases, discovered only once the
  national-team plugin is built] → Explicitly validate the schema against a real Bundesliga
  shape now and note in tasks.md that full validation against EPL/national-team shapes is
  blocked on those plugins existing (tracked as follow-up, not blocking this change).
- [WIT/wit-bindgen tooling instability in the Rust ecosystem, per the umbrella design] →
  Pin `wit-bindgen` version explicitly in this crate; document the pinned version in
  the crate's own changelog so `Apps/rust` and `Plugins/Bundesliga` can match it.
- [Two independent version numbers (schema, interface) is more bookkeeping than one] →
  Accepted trade-off; the alternative (one coupled version) was rejected in Decisions above
  because it forces unrelated breaking changes together.

## Migration Plan

Not applicable — this is a new repo with no prior consumers. `Apps/rust` and
`Plugins/Bundesliga` adopt this contract in their own child changes once it's published.

## Open Questions

- Should the manifest's declared network hosts be validated (format, reachability) at
  parse time in this crate, or left entirely to the host runtime's enforcement? Leaning
  toward format validation only here (host runtime owns enforcement) — confirm when the
  `Apps/rust` host runtime child change is proposed.
