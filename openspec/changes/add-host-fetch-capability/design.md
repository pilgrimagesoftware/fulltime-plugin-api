## Context

`wit/data-provider.wit` defines `world plugin { export data-provider; }` — no imports. A
plugin component built against this world needs nothing from its host to instantiate,
which contradicts the umbrella plugin architecture's premise (plugins are sandboxed, with
no direct network access; all upstream calls go through a host-provided `fetch`
capability) and this crate's own `docs/plugin-authoring.md`, which already describes that
capability as if it exists.

Separately, `src/bindings.rs` runs `wit_bindgen::generate!` inside a private `mod
bindings;` and `src/lib.rs` only re-exports two of its interfaces (`types`, `errors`) as
plain Rust structs/enums. Nothing exposes the `Guest` trait or `export!` macro
`wit_bindgen::generate!` also produces for the `data-provider` export — the parts a plugin
actually needs to *implement* the world, as opposed to just referencing its data shapes.
`Plugins/Bundesliga` hit this directly: its `src/provider.rs` implements the five
operations as plain functions with matching signatures, not as a real `Guest` impl,
because there was nothing to implement against.

## Goals / Non-Goals

**Goals:**
- Define the `host.fetch` WIT import and wire it into `world plugin`.
- Expose enough of this crate's generated bindings that a downstream plugin can depend on
  it as an ordinary Rust library and get a real `Guest` trait, `export!` macro, and a safe
  wrapper around calling `fetch`, without regenerating its own (nominally incompatible)
  copy of the same types.
- Keep the fix backward-referenceable: `Plugins/Bundesliga`'s existing `Fetcher`
  trait/`provider.rs` functions should map cleanly onto the new `Guest` trait once that
  repo does its own follow-up migration (out of scope here, tracked there).

**Non-Goals:**
- Implementing the host side of `fetch` (belongs to `Apps/rust`'s `plugin-host-runtime`
  change).
- Migrating `Plugins/Bundesliga` onto the new bindings (follow-up in that repo).
- Publishing a new crates.io release as part of this change (see proposal.md's Impact
  section).

## Decisions

**`host.fetch` is GET-only, returns raw bytes, and reuses `errors.network-failure` for its
failure case rather than a new error type.**
Rationale: every current and near-term data-provider operation (`openligadb` and the
umbrella design's other planned providers) only needs GET. Returning raw `list<u8>` keeps
the host interface agnostic to response format — deserialization stays the plugin's job,
consistent with `errors`' existing separation between transport failures (`network-failure`)
and mapping failures (`schema-mapping-failure`). A non-2xx HTTP status is folded into
`network-failure` (its `message` field carries the status/detail) rather than adding a
distinct variant now; if a plugin later needs to branch on status code specifically, that's
an additive change to the `host` interface, not a reason to block this one.

```wit
interface host {
    use errors.{network-failure};

    /// Fetches the response body for an HTTP GET request to `url`, via the host.
    ///
    /// The host scopes this to hosts declared in the plugin's manifest
    /// `network_hosts` field; a request to an undeclared host fails as a
    /// `network-failure`, not a distinct permission-denied variant, since from the
    /// plugin's perspective both are "the request didn't succeed."
    fetch: func(url: string) -> result<list<u8>, network-failure>;
}

world plugin {
    import host;
    export data-provider;
}
```

**Alternative considered:** a `Fetcher`-style resource/trait-object import (mirroring
`Plugins/Bundesliga`'s current Rust-side `Fetcher` trait) instead of a single free
function. Rejected: WIT resources add instantiation complexity (the host would need to
construct and pass a resource handle) for no benefit here — there is exactly one
operation, and a free function is the simplest shape that satisfies it.

**`INTERFACE_VERSION` bumps from `1.0` to `2.0`, not `1.1`.**
Rationale: `Version::accepts` (major equal, host minor ≥ plugin minor) models the export
side of compatibility — the host is a superset of the functions a plugin expects to call
*on itself*. Adding a required import inverts that relationship for a new axis: a plugin
built against the old `1.x` world needs nothing from the host to instantiate; a plugin
built against the new world requires the host to supply `fetch`, and there is no way for
an old host to satisfy that later without changing its own code. That is exactly what a
major version bump communicates, even though the *mechanism* enforcing it differs — a
version mismatch is caught by the manifest check in Rust before instantiation, whereas a
missing WIT import is caught by the component linker at instantiation time with a less
informative error. Bumping major means a host built for `1.x` correctly refuses a `2.x`
plugin manifest before ever attempting to instantiate it, rather than surfacing a raw
linker error.
**Open question, not resolved here:** whether `Version::accepts`'s major-match rule should
eventually distinguish "export-shape compatible" from "import-requirements compatible" as
two separate fields, given they're conceptually different axes that happen to share one
version number today. Flagging for the `fulltime-plugin-api` maintainer; not blocking this
change, since collapsing them into one major bump is still correct, just coarser than it
could be.

**`bindings` stays a private module; `export!`, `Guest`, and the generated `exports` tree
are re-exported — more of the generated surface than originally planned.**
The `mod bindings;` module itself stays private, and `pub use bindings::export;` plus a
`Guest` alias were the intended minimal surface. In practice, `wit_bindgen`'s single-arg
`export!($ty)` macro expands to `self::export!($ty with_types_in self)` — `self` resolves
to the *caller's* module, so it only compiles when the macro is invoked inside this crate
itself. A downstream crate must use the macro's `with_types_in <path>` form
(`fulltime_plugin_api::export!(MyPlugin with_types_in fulltime_plugin_api)`), and that form
requires the full generated `exports::fulltime::plugin_api::data_provider` module path to
be reachable at that root — which means `exports` itself has to be `pub use`d, not just the
`Guest` trait pulled out of it. Verified end-to-end by cross-compiling a scratch crate
depending on this one for `wasm32-wasip2`; the single-item re-export alone produced
`cannot find export in self`, `with_types_in` alone then produced `cannot find exports in
fulltime_plugin_api`, and only re-exporting `exports` as well resolved both. This exposes
more of `wit_bindgen`'s generated internals (the full `exports` module tree, not just
`Guest`) than the minimal-surface goal above intended — an acceptable trade because the
alternative (no working `export!` for any downstream crate) defeats the point of this
change entirely, but worth a future `wit-bindgen` version bump checking this path shape
hasn't changed.

**`host_fetch` is a thin wrapper, not a trait.**
Rationale: `wit_bindgen::generate!`'s import binding for `host.fetch` is already a plain
function once compiled for `wasm32` with the component model; wrapping it in
`pub fn host_fetch(url: &str) -> Result<Vec<u8>, NetworkFailure>` just gives it this
crate's own `NetworkFailure` type at the boundary (translating from the generated
`errors::network-failure` type, which is the same struct after the `errors` interface
re-export, so this is close to a no-op today, but keeps the wrapper's signature stable if
the WIT error shape changes later).
**Consequence a downstream plugin must handle itself, documented in
`docs/plugin-authoring.md`:** `host_fetch` only links successfully when the crate is
compiled for `wasm32` as part of a real component instantiated by a host implementing
`host.fetch` — call it from `#[cfg(target_arch = "wasm32")]`-gated code, and keep a
separate injectable seam (as `Plugins/Bundesliga`'s `Fetcher` trait already does) for
native unit/integration tests, with a `wasm32`-only impl of that seam delegating to
`host_fetch`. This crate's own CI does not call `host_fetch` anywhere, so this change does
not by itself require any native-vs-wasm build split in this repo — only in a plugin that
uses it.

## Risks / Trade-offs

- [Every plugin currently built against the `1.x` world (in practice, only
  `Plugins/Bundesliga`'s in-progress, non-exported placeholder) needs a follow-up
  migration once this ships] → No published plugin exists yet; `Plugins/Bundesliga`'s
  migration is tracked as follow-up work in that repo, not blocking this change.
- [`host_fetch` looks callable natively but only actually links inside a real wasm
  component] → Documented explicitly in `docs/plugin-authoring.md` and in the wrapper's
  own doc comment; the design's `#[cfg(target_arch = "wasm32")]` guidance is the mitigation.
- [Collapsing "export compatible" and "import compatible" into one `INTERFACE_VERSION`
  major number is coarser than strictly necessary] → Flagged as an open question above
  rather than solved speculatively; revisit if a future import-only or export-only change
  makes the coupling actually costly.

## Migration Plan

Not applicable in the deployment sense — no host runtime consumes this crate yet. The
migration that matters is source-level: `Plugins/Bundesliga` (and any future plugin)
switches from a self-defined `Fetcher`-style seam to this crate's `Guest`/`export!`/
`host_fetch`, tracked as follow-up work in that repo once this change is merged.

## Open Questions

- Should `Version::accepts` eventually split into separate export-compatibility and
  import-compatibility checks? Not resolved here (see the `INTERFACE_VERSION` decision
  above) — flagging for the maintainer rather than deciding unilaterally in this change.
