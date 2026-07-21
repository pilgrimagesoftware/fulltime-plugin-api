## Context

The plugin manifest (`src/manifest.rs`) currently carries `id`, `version`, `schema_version`,
`interface_version`, and `network_hosts` — enough for the host to load and version-check a
plugin, but nothing to show a human which developer/publisher built it or when. `Apps/rust`'s
Plugins management screen (`openspec/changes/plugin-host-runtime`, already implemented there)
lists `id` and `version` only, for exactly this reason.

Two prior manifest fields established a validation precedent worth following here:
`schema_version`/`interface_version` are parsed into a typed [`Version`] because the host
actively compares them for compatibility. `network_hosts` entries get only a "must not be empty"
check because they're used as-is (string equality against a request's host). This change's two
new fields are purely informational — nothing compares or parses them — so they follow the
`network_hosts` precedent, not the `Version` one.

## Goals / Non-Goals

**Goals:**
- Let a plugin manifest optionally declare a developer/publisher display name and a build
  timestamp.
- Keep every existing manifest (in particular `Plugins/Bundesliga`'s) parsing unchanged with no
  edits required — an additive, minor-version change per this crate's own versioning policy.

**Non-Goals:**
- Validating `build_date` as a well-formed timestamp. This crate never validates `Fixture.kickoff`
  (also documented as RFC 3339) either; adding parsing here would be inconsistent and would pull
  in a date/time dependency (`time` or `chrono`) this crate has never needed, bloating every
  plugin's compiled `wasm32` component for a display-only field.
- Any host-side or UI-side consumption of these fields. Surfacing them in `Apps/rust`'s Plugins
  screen is a separate, follow-up change in that repo.
- Making either field required. That would be a breaking, major-version change forcing every
  existing plugin (starting with `Plugins/Bundesliga`) to update its manifest before it could be
  loaded by a host built against the new version.

## Decisions

**Both fields are `Option<String>`, not a new typed wrapper.** `developer` is a free-form display
string (no format to validate beyond non-empty). `build_date` is documented as RFC 3339 but stored
and returned as the raw string, exactly like `Fixture.kickoff` — this crate parses neither.
Alternative considered: a `Version`-style typed date wrapper with parse validation, rejected per
the Non-Goals above (inconsistent with `kickoff`, needless dependency, no consumer that needs a
parsed value yet).

**Both fields are optional, not required.** Alternative considered: required fields, rejected
because it forces a major version bump and breaks every existing manifest, for two fields whose
absence is a completely reasonable state (a plugin author who hasn't set up a build-date stamping
step yet, or doesn't want to disclose a developer name).

**Validation mirrors `network_hosts`, not `schema_version`.** When present, each field must be a
non-empty string after trimming (same rule `network_hosts` entries already use) — not a schema
compatibility concern, so no `ManifestField` variant needs special version-parsing logic, just the
same "field is present but empty" rejection path `network_hosts` already has.

## Risks / Trade-offs

- [A future need to actually parse `build_date` (e.g. to sort plugins by recency) would require
  revisiting the no-validation decision] → Acceptable now: no consumer needs a parsed value yet,
  and adding validation later is itself another additive, non-breaking change (tightening an
  `Option<String>` to reject previously-accepted malformed strings would be the only breaking
  edge case, and is deferred to if/when it's actually needed).
- [`developer` has no format constraint at all, so two plugins could declare visually-identical or
  confusingly-similar developer names] → Out of scope: this crate validates manifest structure,
  not developer identity or trust — matching its existing stated non-goal for `network_hosts`
  ("this crate validates manifest format only").

## Migration Plan

1. Add both fields to `RawManifest` and `Manifest`, both `Option<String>`, with the non-empty
   check applied only when present.
2. Bump `Cargo.toml`'s version per this being an additive/minor change (handled by the normal
   `git-cliff`-driven release process in `RELEASING.md`, not a manual step here).
3. No manifest anywhere needs to change for this to ship — `Plugins/Bundesliga`'s current
   `manifest.toml` keeps parsing exactly as it does today, with both new fields resolving to
   `None`.

Rollback: revert the two-field addition; no data migration exists since nothing is persisted by
this crate itself.

## Open Questions

- Should `Apps/rust`'s Plugins screen surface these fields once available? Deferred to a
  follow-up change in that repo, coordinated after this one ships and a new `fulltime-plugin-api`
  version is cut.
