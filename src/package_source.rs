/// Describes how a package should be resolved: either from a local path on
/// disk or by fetching a specific version via npm.
pub enum PackageSource {
    /// An absolute path to an already-installed package directory.
    Path(String),
    /// A version string passed to npm, e.g. `"17.3.0"` or `"latest"`.
    Version(String),
}

impl PackageSource {
    /// Parses a raw settings string into a [`PackageSource`].
    ///
    /// Strings starting with `/` or `./` are treated as local paths;
    /// everything else is treated as a version identifier.
    pub fn from_str(s: &str) -> Self {
        if s.starts_with('/') || s.starts_with("./") {
            Self::Path(s.to_string())
        } else {
            Self::Version(s.to_string())
        }
    }
}

#[cfg(test)]
mod package_source_tests {
    use super::*;

    #[test]
    fn test_absolute_path_is_recognized() {
        assert!(matches!(
            PackageSource::from_str("/usr/local/lib/node_modules/typescript"),
            PackageSource::Path(_)
        ));
    }

    #[test]
    fn test_relative_path_is_recognized() {
        assert!(matches!(
            PackageSource::from_str("./local/typescript"),
            PackageSource::Path(_)
        ));
    }

    #[test]
    fn test_version_string_is_recognized() {
        assert!(matches!(
            PackageSource::from_str("17.3.0"),
            PackageSource::Version(_)
        ));
    }

    #[test]
    fn test_latest_is_a_version() {
        assert!(matches!(
            PackageSource::from_str("latest"),
            PackageSource::Version(_)
        ));
    }

    #[test]
    fn test_path_value_is_preserved() {
        let path = "/usr/local/lib/typescript";
        let PackageSource::Path(value) = PackageSource::from_str(path) else {
            panic!("expected Path");
        };
        assert_eq!(value, path);
    }

    #[test]
    fn test_version_value_is_preserved() {
        let version = "17.3.0";
        let PackageSource::Version(value) = PackageSource::from_str(version) else {
            panic!("expected Version");
        };
        assert_eq!(value, version);
    }
}
