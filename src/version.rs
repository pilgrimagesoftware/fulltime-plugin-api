//! Independent `major.minor` version identifiers for the canonical schema and the
//! data-provider interface, plus the consumer-compatibility check the host runs before
//! loading a plugin.
//!
//! See `openspec/changes/define-league-data-contract/design.md` ("Schema version and
//! interface version are tracked independently") and both specs' "Versioning" scenarios.

use core::fmt;
use core::str::FromStr;

/// A `major.minor` version identifier for either the canonical schema or the
/// data-provider interface.
///
/// # Examples
///
/// ```
/// use fulltime_plugin_api::Version;
///
/// let host = Version::new(1, 3);
/// let plugin = Version::new(1, 2);
/// assert!(host.accepts(plugin));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// Major version. A mismatch is always incompatible.
    pub major: u16,
    /// Minor version. A host may run a plugin targeting an equal or lower minor version.
    pub minor: u16,
}

impl Version {
    /// Creates a new version identifier.
    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns whether `self`, acting as the host's supported version, accepts a plugin
    /// declaring `target` as the version it was built against.
    ///
    /// Compatible when the major versions match and the host's minor version is equal to
    /// or greater than the plugin's, since a higher host minor version is a superset of
    /// the fields or functions the plugin was built against.
    ///
    /// # Examples
    ///
    /// ```
    /// use fulltime_plugin_api::Version;
    ///
    /// assert!(Version::new(1, 3).accepts(Version::new(1, 2)));
    /// assert!(!Version::new(1, 1).accepts(Version::new(1, 2)));
    /// assert!(!Version::new(2, 0).accepts(Version::new(1, 9)));
    /// ```
    #[must_use]
    pub const fn accepts(self, target: Self) -> bool {
        self.major == target.major && self.minor >= target.minor
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Error returned when parsing a [`Version`] from a string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid version string {0:?}, expected \"major.minor\"")]
pub struct ParseVersionError(String);

impl FromStr for Version {
    type Err = ParseVersionError;

    /// Parses a `"major.minor"` version string.
    ///
    /// # Errors
    ///
    /// Returns [`ParseVersionError`] if the string is not exactly two `u16` components
    /// separated by a single `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fulltime_plugin_api::Version;
    ///
    /// assert_eq!("1.2".parse(), Ok(Version::new(1, 2)));
    /// assert!("1".parse::<Version>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s
            .split_once('.')
            .ok_or_else(|| ParseVersionError(s.to_owned()))?;
        let major = major.parse().map_err(|_| ParseVersionError(s.to_owned()))?;
        let minor = minor.parse().map_err(|_| ParseVersionError(s.to_owned()))?;
        Ok(Self { major, minor })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_same_major_lower_minor() {
        assert!(Version::new(1, 3).accepts(Version::new(1, 2)));
    }

    #[test]
    fn rejects_same_major_higher_minor() {
        assert!(!Version::new(1, 1).accepts(Version::new(1, 2)));
    }

    #[test]
    fn rejects_different_major() {
        assert!(!Version::new(2, 0).accepts(Version::new(1, 9)));
    }

    #[test]
    fn round_trips_through_display_and_parse() {
        let v = Version::new(3, 7);
        assert_eq!(v.to_string().parse(), Ok(v));
    }

    #[test]
    fn rejects_malformed_strings() {
        assert!("1".parse::<Version>().is_err());
        assert!("1.2.3".parse::<Version>().is_err());
        assert!("a.b".parse::<Version>().is_err());
    }
}
