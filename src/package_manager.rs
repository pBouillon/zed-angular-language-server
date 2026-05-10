use serde::Deserialize;
use std::collections::HashMap;
use zed_extension_api as zed;

use crate::semantic_version::SemanticVersion;

/// The name of the Angular core package, used to determine the project's
/// version.
const ANGULAR_CORE_PACKAGE: &str = "@angular/core";

/// The default version to use if no specific version can be detected.
const DEFAULT_VERSION: &str = "latest";

/// The path to the package.json file relative to the worktree root.
const PACKAGE_JSON_PATH: &str = "package.json";

/// The name of theTypeScript package, used to run the Angular Language Server.
const TYPESCRIPT_PACKAGE: &str = "typescript";

/// Represents a subset of package.json to extract dependency versions.
#[derive(Deserialize)]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, String>,
}

/// Represents the resolved versions needed for the language server.
pub struct AngularProjectVersions {
    pub angular: String,
    pub typescript: String,
}

/// Detects the packages version from the project's package.json.
pub fn detect_project_versions(worktree: &zed::Worktree) -> AngularProjectVersions {
    let json = worktree
        .read_text_file(PACKAGE_JSON_PATH)
        .ok()
        .and_then(|content| serde_json::from_str::<PackageJson>(&content).ok());

    let angular_version = json
        .as_ref()
        .and_then(|json| get_package_version(ANGULAR_CORE_PACKAGE, json));
    let typescript_version = json
        .as_ref()
        .and_then(|json| get_package_version(TYPESCRIPT_PACKAGE, json));

    let angular = angular_version
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| DEFAULT_VERSION.to_string());

    let typescript = typescript_version
        .map(|v| v.to_string())
        .unwrap_or_else(|| get_compatible_ts_version_with(angular_version.as_ref()));

    AngularProjectVersions {
        angular,
        typescript,
    }
}

/// Retrieves a compatible TypeScript version with the provided version of
/// Angular according to the
/// [version compatibility matrix][https://angular.dev/reference/versions].
fn get_compatible_ts_version_with(angular_version: Option<&SemanticVersion>) -> String {
    let Some(v) = angular_version else {
        return DEFAULT_VERSION.to_string();
    };

    match (v.major, v.minor) {
        (21, _) => "5.9.0", // >=5.9.0 <6.0.0
        (20, _) => "5.8.0", // >=5.8.0 <6.0.0
        (19, _) => "5.5.0", // >=5.5.0 <5.9.0
        (18, _) => "5.4.0", // >=5.4.0 <5.6.0
        (17, _) => "5.2.0", // >=5.2.0 <5.5.0
        (16, _) => "4.9.3", // >=4.9.3 <5.2.0
        (15, _) => "4.8.2", // >=4.8.2 <5.0.0
        (14, _) => "4.6.2", // >=4.6.2 <4.9.0
        (13, _) => "4.4.3", // >=4.4.3 <4.7.0
        (12, _) => "4.2.3", // >=4.2.3 <4.4.0
        _ => DEFAULT_VERSION,
    }
    .to_string()
}

/// Retrieves the version of a given package in a package.json.
fn get_package_version(package_name: &str, json: &PackageJson) -> Option<SemanticVersion> {
    json.dependencies
        .get(package_name)
        .or_else(|| json.dev_dependencies.get(package_name))
        .map(|v| SemanticVersion::parse(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_package_json(
        dependencies: &[(&str, &str)],
        dev_dependencies: &[(&str, &str)],
    ) -> PackageJson {
        PackageJson {
            dependencies: dependencies
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            dev_dependencies: dev_dependencies
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }

    #[test]
    fn test_get_package_version_found_in_dependencies() {
        let json = make_package_json(&[("typescript", "5.4.0")], &[]);
        assert_eq!(
            get_package_version("typescript", &json)
                .unwrap()
                .to_string(),
            "5.4.0"
        );
    }

    #[test]
    fn test_get_package_version_found_in_dev_dependencies() {
        let json = make_package_json(&[], &[("typescript", "5.4.0")]);
        assert_eq!(
            get_package_version("typescript", &json)
                .unwrap()
                .to_string(),
            "5.4.0"
        );
    }

    #[test]
    fn test_get_package_version_dependencies_takes_precedence_over_dev() {
        let json = make_package_json(&[("typescript", "5.4.0")], &[("typescript", "4.0.0")]);
        assert_eq!(
            get_package_version("typescript", &json)
                .unwrap()
                .to_string(),
            "5.4.0"
        );
    }

    #[test]
    fn test_get_package_version_not_found() {
        let json = make_package_json(&[], &[]);
        assert!(get_package_version("typescript", &json).is_none());
    }

    #[test]
    fn test_compatible_ts_version_no_angular_version() {
        assert_eq!(get_compatible_ts_version_with(None), DEFAULT_VERSION);
    }

    #[test]
    fn test_compatible_ts_version_unknown_angular_version() {
        let version = SemanticVersion::parse("1.0.0");
        assert_eq!(
            get_compatible_ts_version_with(Some(&version)),
            DEFAULT_VERSION
        );
    }
}
