//! Canonical league-data schema, data-provider WIT interface, and plugin manifest format
//! shared by the `FullTime` plugin host and every data-provider plugin.
//!
//! Neither the host nor any plugin owns this contract: it is versioned and published
//! independently of both so the host and plugins can evolve without a lockstep release.
//! See `openspec/changes/define-league-data-contract/proposal.md` for the full rationale.
//!
//! # Examples
//!
//! ```
//! use fulltime_plugin_api::{Manifest, SCHEMA_VERSION};
//!
//! let source = include_str!("../tests/fixtures/manifest.toml");
//! let manifest = Manifest::parse(source)?;
//! assert!(SCHEMA_VERSION.accepts(manifest.schema_version));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![warn(clippy::pedantic, clippy::nursery, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]
#![forbid(unsafe_code)]

mod bindings;
mod manifest;
mod version;

pub use manifest::{Manifest, ManifestError, ManifestField};
pub use version::{ParseVersionError, Version};

pub use bindings::fulltime::plugin_api::errors::*;
pub use bindings::fulltime::plugin_api::types::*;

pub use bindings::export;
pub use bindings::exports::fulltime::plugin_api::data_provider::Guest;

/// Fetches the response body for an HTTP GET request to `url`, via the host's `fetch`
/// capability.
///
/// This only links and behaves correctly when compiled as part of a `wasm32` component
/// instantiated by a host implementing the `host` interface's `fetch` function (see
/// `wit/data-provider.wit`) — it has no behavior of its own independent of that generated
/// import. Callers should gate use of it behind `#[cfg(target_arch = "wasm32")]` and keep
/// a separate, injectable seam for native unit/integration tests.
///
/// # Errors
/// Returns [`NetworkFailure`] if the host reports the request failed (network error,
/// non-2xx status, or a plugin-manifest network-host restriction).
pub fn host_fetch(url: &str) -> Result<Vec<u8>, NetworkFailure> {
    bindings::fulltime::plugin_api::host::fetch(url)
}

/// Current version of the canonical `league-data-schema` (see [`types`
/// interface](https://github.com/pilgrimagesoftware/fulltime-plugin-api/blob/develop/wit/data-provider.wit)).
///
/// A plugin declaring a `schema_version` in its manifest is compatible with a host running
/// this version when `SCHEMA_VERSION.accepts(plugin_schema_version)` is `true` — see
/// [`Version::accepts`].
pub const SCHEMA_VERSION: Version = Version::new(1, 0);

/// Current version of the `data-provider` WIT interface.
///
/// A plugin declaring an `interface_version` in its manifest is compatible with a host
/// running this version when `INTERFACE_VERSION.accepts(plugin_interface_version)` is
/// `true` — see [`Version::accepts`].
pub const INTERFACE_VERSION: Version = Version::new(2, 0);
