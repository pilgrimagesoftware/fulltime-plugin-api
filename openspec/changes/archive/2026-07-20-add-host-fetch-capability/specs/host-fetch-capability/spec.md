## ADDED Requirements

### Requirement: Host Fetch WIT Import
The `plugin` world SHALL import a `host` interface defining a `fetch` function, so a
plugin component cannot instantiate against a host that does not supply network access.

#### Scenario: Host implements fetch
- **WHEN** a host loads a plugin component built against the `plugin` world
- **THEN** instantiation requires the host to supply an implementation of `host.fetch`

#### Scenario: Plugin makes an HTTP GET request
- **WHEN** a plugin needs data from an upstream HTTP API
- **THEN** it calls `host.fetch` with the target URL and receives either the response body
  or a `network-failure` error, and issues no direct network connection of its own

### Requirement: Fetch Errors Reuse the Existing Error Shape
`host.fetch` SHALL report failures using the `errors` interface's existing
`network-failure` record rather than a separate error type.

#### Scenario: Upstream request fails
- **WHEN** `host.fetch` cannot complete the request (network error, non-2xx status, or a
  host-enforced network-host restriction from the plugin's manifest)
- **THEN** it returns `network-failure` with a message describing the failure, and the
  plugin handles it identically to a `network-failure` from any other source

### Requirement: Rust Wrapper Around the Generated Import
This crate SHALL expose a safe Rust function wrapping the generated `host.fetch` import,
so a plugin calls ordinary Rust rather than raw `wit_bindgen`-generated bindings.

#### Scenario: Plugin calls the wrapper
- **WHEN** a plugin compiled as a `wasm32` component calls this crate's `host_fetch`
  function
- **THEN** the call resolves to the generated `host.fetch` import and returns
  `Result<Vec<u8>, NetworkFailure>` using this crate's own re-exported `NetworkFailure`
  type

#### Scenario: Wrapper called outside a real component instantiation
- **WHEN** `host_fetch` is referenced from code compiled for a non-`wasm32` target, or
  from a `wasm32` build not instantiated by a host implementing `host.fetch`
- **THEN** the call fails to link or resolve, since the wrapper has no behavior of its own
  independent of the generated import — callers are documented to gate use of it behind
  `#[cfg(target_arch = "wasm32")]` and keep a separate, injectable seam for native tests
