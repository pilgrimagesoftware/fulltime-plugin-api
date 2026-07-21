//! Plugin manifest format: the static, TOML-encoded file every plugin ships declaring its
//! identity, targeted contract versions, and required network hosts.
//!
//! This module validates manifest structure and field presence/format only. It performs
//! no host-side enforcement (network reachability, capability granting, enable/disable
//! state) — that belongs to the plugin host runtime. See
//! `openspec/changes/define-league-data-contract/specs/plugin-manifest-format/spec.md`.

use serde::Deserialize;

use crate::version::Version;

/// A parsed plugin manifest.
///
/// # Examples
///
/// ```
/// use fulltime_plugin_api::Manifest;
///
/// let toml = r#"
///     id = "bundesliga"
///     name = "Bundesliga"
///     version = "0.1.0"
///     schema_version = "1.0"
///     interface_version = "1.0"
///     network_hosts = ["api.openligadb.de"]
/// "#;
///
/// let manifest = Manifest::parse(toml).unwrap();
/// assert_eq!(manifest.id, "bundesliga");
/// assert_eq!(manifest.name, "Bundesliga");
/// assert_eq!(manifest.network_hosts, ["api.openligadb.de"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Plugin identifier, unique among plugins the host loads.
    pub id: String,
    /// Human-readable display name (e.g. `"Bundesliga"`), distinct from
    /// `id`. A plugin manifest is the only place this is declared — hosts
    /// must not derive a display name from `id` (e.g. by title-casing it).
    pub name: String,
    /// Plugin's own release version (not a contract version).
    pub version: String,
    /// Canonical schema version this plugin's output targets.
    pub schema_version: Version,
    /// Data-provider interface version this plugin was built against.
    pub interface_version: Version,
    /// Network hosts this plugin requires access to.
    pub network_hosts: Vec<String>,
}

/// A manifest field that failed presence or format validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestField {
    /// The `id` field.
    Id,
    /// The `name` field.
    Name,
    /// The `version` field.
    Version,
    /// The `schema_version` field.
    SchemaVersion,
    /// The `interface_version` field.
    InterfaceVersion,
    /// The `network_hosts` field.
    NetworkHosts,
}

impl core::fmt::Display for ManifestField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Version => "version",
            Self::SchemaVersion => "schema_version",
            Self::InterfaceVersion => "interface_version",
            Self::NetworkHosts => "network_hosts",
        };
        f.write_str(name)
    }
}

/// Error returned when a manifest fails to parse.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    /// The manifest is not well-formed TOML.
    #[error("manifest is not valid TOML: {0}")]
    Malformed(#[from] toml::de::Error),

    /// A required field is missing, or a present field has an invalid format.
    #[error("invalid manifest field {field}: {reason}")]
    InvalidField {
        /// The field that failed validation.
        field: ManifestField,
        /// Human-readable reason, safe to surface to a plugin author.
        reason: String,
    },
}

/// Raw, unvalidated manifest shape as it appears on disk.
#[derive(Debug, Deserialize)]
struct RawManifest {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    schema_version: Option<String>,
    interface_version: Option<String>,
    network_hosts: Option<Vec<String>>,
}

impl Manifest {
    /// Parses and validates a manifest from its TOML source.
    ///
    /// # Errors
    ///
    /// Returns [`ManifestError::Malformed`] if `source` is not valid TOML, or
    /// [`ManifestError::InvalidField`] if a required field is missing or a version field
    /// is not a valid `"major.minor"` string. No network host in `network_hosts` is
    /// contacted or otherwise validated beyond being a non-empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// use fulltime_plugin_api::{Manifest, ManifestError};
    ///
    /// let err = Manifest::parse("id = \"x\"").unwrap_err();
    /// assert!(matches!(err, ManifestError::InvalidField { .. }));
    /// ```
    pub fn parse(source: &str) -> Result<Self, ManifestError> {
        let raw: RawManifest = toml::from_str(source)?;

        let id = required(raw.id, ManifestField::Id)?;
        let name = required(raw.name, ManifestField::Name)?;
        let version = required(raw.version, ManifestField::Version)?;
        let schema_version = parse_version(raw.schema_version, ManifestField::SchemaVersion)?;
        let interface_version =
            parse_version(raw.interface_version, ManifestField::InterfaceVersion)?;
        let network_hosts = required(raw.network_hosts, ManifestField::NetworkHosts)?;

        if name.trim().is_empty() {
            return Err(ManifestError::InvalidField {
                field: ManifestField::Name,
                reason: "name must not be empty".to_owned(),
            });
        }

        if network_hosts.iter().any(|host| host.trim().is_empty()) {
            return Err(ManifestError::InvalidField {
                field: ManifestField::NetworkHosts,
                reason: "network_hosts entries must not be empty".to_owned(),
            });
        }

        Ok(Self {
            id,
            name,
            version,
            schema_version,
            interface_version,
            network_hosts,
        })
    }
}

fn required<T>(value: Option<T>, field: ManifestField) -> Result<T, ManifestError> {
    value.ok_or_else(|| ManifestError::InvalidField {
        field,
        reason: "field is required".to_owned(),
    })
}

fn parse_version(value: Option<String>, field: ManifestField) -> Result<Version, ManifestError> {
    let raw = required(value, field)?;
    raw.parse().map_err(|_| ManifestError::InvalidField {
        field,
        reason: format!("{raw:?} is not a valid \"major.minor\" version"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_toml() -> &'static str {
        r#"
            id = "bundesliga"
            name = "Bundesliga"
            version = "0.1.0"
            schema_version = "1.0"
            interface_version = "1.0"
            network_hosts = ["api.openligadb.de"]
        "#
    }

    #[test]
    fn parses_a_well_formed_manifest() {
        let manifest = Manifest::parse(valid_toml()).unwrap();
        assert_eq!(manifest.id, "bundesliga");
        assert_eq!(manifest.name, "Bundesliga");
        assert_eq!(manifest.schema_version, Version::new(1, 0));
        assert_eq!(manifest.network_hosts, vec!["api.openligadb.de".to_owned()]);
    }

    #[test]
    fn rejects_missing_required_field() {
        let err = Manifest::parse("id = \"x\"").unwrap_err();
        assert!(matches!(
            err,
            ManifestError::InvalidField {
                field: ManifestField::Name,
                ..
            }
        ));
    }

    #[test]
    fn rejects_empty_name() {
        let toml = r#"
            id = "bundesliga"
            name = "   "
            version = "0.1.0"
            schema_version = "1.0"
            interface_version = "1.0"
            network_hosts = ["api.openligadb.de"]
        "#;
        let err = Manifest::parse(toml).unwrap_err();
        assert!(matches!(
            err,
            ManifestError::InvalidField {
                field: ManifestField::Name,
                ..
            }
        ));
    }

    #[test]
    fn rejects_malformed_version_string() {
        let toml = r#"
            id = "bundesliga"
            name = "Bundesliga"
            version = "0.1.0"
            schema_version = "not-a-version"
            interface_version = "1.0"
            network_hosts = ["api.openligadb.de"]
        "#;
        let err = Manifest::parse(toml).unwrap_err();
        assert!(matches!(
            err,
            ManifestError::InvalidField {
                field: ManifestField::SchemaVersion,
                ..
            }
        ));
    }

    #[test]
    fn rejects_empty_network_host_entry() {
        let toml = r#"
            id = "bundesliga"
            name = "Bundesliga"
            version = "0.1.0"
            schema_version = "1.0"
            interface_version = "1.0"
            network_hosts = [""]
        "#;
        let err = Manifest::parse(toml).unwrap_err();
        assert!(matches!(
            err,
            ManifestError::InvalidField {
                field: ManifestField::NetworkHosts,
                ..
            }
        ));
    }

    #[test]
    fn rejects_malformed_toml() {
        let err = Manifest::parse("not = [valid").unwrap_err();
        assert!(matches!(err, ManifestError::Malformed(_)));
    }

    #[test]
    fn does_not_contact_declared_network_hosts() {
        // Parsing a manifest declaring an unreachable/nonexistent host must still succeed;
        // this crate performs format validation only.
        let toml = r#"
            id = "x"
            name = "X"
            version = "0.1.0"
            schema_version = "1.0"
            interface_version = "1.0"
            network_hosts = ["definitely-not-a-real-host.invalid"]
        "#;
        assert!(Manifest::parse(toml).is_ok());
    }

    #[test]
    fn interface_version_2_0_is_accepted_by_the_current_interface_version() {
        let toml = r#"
            id = "bundesliga"
            name = "Bundesliga"
            version = "0.1.0"
            schema_version = "1.0"
            interface_version = "2.0"
            network_hosts = ["api.openligadb.de"]
        "#;
        let manifest = Manifest::parse(toml).unwrap();
        assert_eq!(manifest.interface_version, Version::new(2, 0));
        assert!(crate::INTERFACE_VERSION.accepts(manifest.interface_version));
    }

    #[test]
    fn interface_version_1_0_is_rejected_after_the_host_fetch_major_bump() {
        // A plugin built before `host.fetch` existed declares interface_version 1.0; the
        // host's INTERFACE_VERSION is now 2.0 (major bump), so it must not accept it — see
        // openspec/changes/add-host-fetch-capability/specs/data-provider-plugin-api/spec.md
        // ("Plugin built before the host-fetch import existed").
        let manifest = Manifest::parse(valid_toml()).unwrap();
        assert_eq!(manifest.interface_version, Version::new(1, 0));
        assert!(!crate::INTERFACE_VERSION.accepts(manifest.interface_version));
    }
}
