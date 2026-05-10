/// A parsed semantic version with major, minor, and patch components.
pub struct SemanticVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl SemanticVersion {
    /// Parses a version string into a SemanticVersion.
    ///
    /// This handles common npm prefixes by stripping non-digit characters
    /// from the start of the string.
    pub fn parse(version: &str) -> Self {
        let stripped = version.trim_start_matches(|c: char| !c.is_ascii_digit());
        let mut parts = stripped.split('.');
        let mut next = || parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);

        Self {
            major: next(),
            minor: next(),
            patch: next(),
        }
    }
}

impl std::fmt::Display for SemanticVersion {
    /// Formats the semantic version as `major.minor.patch`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_standard_version() {
        let v = SemanticVersion::parse("1.2.3");
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_parse_with_prefixes() {
        let v1 = SemanticVersion::parse("v2.0.0");
        assert_eq!((v1.major, v1.minor, v1.patch), (2, 0, 0));

        let v2 = SemanticVersion::parse("^1.4.2");
        assert_eq!((v2.major, v2.minor, v2.patch), (1, 4, 2));

        let v3 = SemanticVersion::parse("~3.1.0");
        assert_eq!((v3.major, v3.minor, v3.patch), (3, 1, 0));
    }

    #[test]
    fn test_parse_incomplete_versions() {
        let v1 = SemanticVersion::parse("1.2");
        assert_eq!((v1.major, v1.minor, v1.patch), (1, 2, 0));

        let v2 = SemanticVersion::parse("1");
        assert_eq!((v2.major, v2.minor, v2.patch), (1, 0, 0));
    }

    #[test]
    fn test_parse_with_suffixes() {
        let v = SemanticVersion::parse("1.2.3-beta");
        assert_eq!((v.major, v.minor, v.patch), (1, 2, 0));
    }

    #[test]
    fn test_parse_invalid_or_empty_strings() {
        let v1 = SemanticVersion::parse("");
        assert_eq!((v1.major, v1.minor, v1.patch), (0, 0, 0));

        let v2 = SemanticVersion::parse("invalid-string");
        assert_eq!((v2.major, v2.minor, v2.patch), (0, 0, 0));
    }

    #[test]
    fn test_display_trait() {
        let v = SemanticVersion {
            major: 4,
            minor: 5,
            patch: 6,
        };
        assert_eq!(v.to_string(), "4.5.6");
        assert_eq!(format!("{}", v), "4.5.6");
    }
}
