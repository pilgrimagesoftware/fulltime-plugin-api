## 1. WIT Contract

- [x] 1.1 Add the `host` interface to `wit/data-provider.wit` with a `fetch` function
  reusing `errors.network-failure`
- [x] 1.2 Add `import host;` to `world plugin`, alongside the existing `export
  data-provider;`
- [x] 1.3 Bump `INTERFACE_VERSION` in `src/lib.rs` from `Version::new(1, 0)` to
  `Version::new(2, 0)`

## 2. Bindings Re-exports

- [x] 2.1 Re-export the `export!` macro generated for the `data-provider` interface from
  the crate root
- [x] 2.2 Re-export the generated `Guest` trait for `data-provider` under a clear public
  name (e.g. `DataProviderGuest` or `Guest`, matching whichever reads better against the
  existing `pub use bindings::fulltime::plugin_api::{errors, types}::*;` re-exports)
- [x] 2.3 Add a `host_fetch(url: &str) -> Result<Vec<u8>, NetworkFailure>` wrapper around
  the generated `host.fetch` import, gated with a doc comment (not a `cfg`, since this
  crate itself doesn't need to restrict compilation — only callers do) explaining it only
  links inside a `wasm32` component instantiated by a compatible host

## 3. Verification

- [x] 3.1 `cargo build`/`test`/`clippy`/`fmt --check` all pass natively (confirms adding
  the `host` import doesn't break this crate's own non-wasm CI, since nothing in this
  crate's test suite calls `host_fetch`)
- [x] 3.2 Add a unit or doc test exercising `Manifest::parse` against an `interface_version
  = "2.0"` manifest, confirming `INTERFACE_VERSION.accepts` behaves as documented in the
  updated Interface Versioning requirement
- [x] 3.3 `cargo doc --no-deps` builds clean with the new public items documented

## 4. Documentation

- [x] 4.1 Update `docs/plugin-authoring.md` with a section on implementing the `Guest`
  trait, calling `export!`, and using `host_fetch` (including the `#[cfg(target_arch =
  "wasm32")]` + separate native-test-seam guidance from design.md)
- [x] 4.2 Update `README.md`'s "Building a plugin" pointer if the new bindings change what
  it should say
- [x] 4.3 Add a `CHANGELOG.md` `[Unreleased]` entry (BREAKING: `INTERFACE_VERSION` 2.0,
  `host.fetch` import required)
