## Why

The `data-provider` WIT world currently only `export`s the plugin interface — it defines
no host-provided `fetch` import, even though the umbrella plugin architecture and this
crate's own `docs/plugin-authoring.md` require plugins to have no direct network access
and to route all upstream calls through a host `fetch` capability. Separately, this
crate's generated bindings (`mod bindings;`) are private, so a downstream plugin has no
way to obtain the `Guest` trait or `export!` macro needed to actually implement and export
the `data-provider` world — every plugin is currently forced to regenerate its own
bindings from a vendored copy of the WIT file, producing Rust types that are WIT-identical
but nominally distinct from this crate's own `Team`/`Fixture`/etc, defeating the point of
a shared canonical Rust API. The reference plugin (`Plugins/Bundesliga`) has hit both gaps
directly and built a placeholder `Fetcher` trait and plain-function operations as the seam
these fixes are meant to replace.

## What Changes

- Add a `host` WIT interface with a `fetch` function plugins import to make HTTP GET
  requests, returning the response body or a structured error.
- **BREAKING**: Wire `host.fetch` into `world plugin` as an `import`, alongside the
  existing `export data-provider`. Any component built against the `plugin` world now
  requires a host that supplies `fetch` — bump `INTERFACE_VERSION` to `2.0` (see
  design.md for why this is a major, not minor, bump).
- Reuse the existing `errors` interface's `network-failure` record for `fetch`'s failure
  case rather than defining a parallel error shape.
- Make `bindings` a `pub(crate)`-visible generation point whose useful downstream surface
  is re-exported: the `data-provider` interface's `Guest` trait, its `export!` macro, and
  a safe Rust wrapper function around the generated `fetch` import (`host_fetch`) so a
  plugin calls ordinary Rust rather than raw generated bindings.

## Capabilities

### New Capabilities
- `host-fetch-capability`: the WIT `host.fetch` import a plugin uses to make HTTP requests
  through the host, and the Rust wrapper this crate exposes around it.

### Modified Capabilities
- `data-provider-plugin-api`: adds a requirement that this crate expose the means for a
  downstream plugin to implement and export the `data-provider` world using this crate's
  own generated types (not a second, incompatible set from a vendored WIT copy), and bumps
  the interface version policy's example to reflect `INTERFACE_VERSION` moving to `2.0`.

## Impact

- **This repo (`fulltime-plugin-api`)**: `wit/data-provider.wit` gains the `host`
  interface and the `plugin` world import; `src/bindings.rs` and `src/lib.rs` change their
  visibility/re-export surface; `INTERFACE_VERSION` bumps to `2.0`; `docs/plugin-authoring.md`
  gains a section on implementing and exporting against the world using the new re-exports.
- **`Plugins/Bundesliga`**: once this ships, its `src/transport.rs` `Fetcher` trait and
  `src/provider.rs` plain functions are replaced with an implementation against this
  crate's `Guest` trait, `export!` macro, and `host_fetch` wrapper — tracked as follow-up
  work in that repo, not part of this change.
- **`Apps/rust`**: its (not-yet-started) `plugin-host-runtime` change must implement the
  `host.fetch` import when it builds the host runtime; this change defines the contract
  that work implements against, it does not implement the host side.
- No crates.io release is required for this change to unblock `Plugins/Bundesliga`
  locally (a git dependency on this branch/commit is sufficient); publishing `0.2.0` is a
  separate, later action once this is reviewed and merged.
