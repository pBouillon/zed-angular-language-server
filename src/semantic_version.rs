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
