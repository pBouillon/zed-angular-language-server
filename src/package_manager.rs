use std::collections::HashMap;
use serde::Deserialize;
use zed_extension_api as zed;

/// The name of the Angular core package, used to determine the project's
/// version.
const ANGULAR_CORE_PACKAGE: &str = "@angular/core";

/// The default version to use if no specific version can be detected.
const DEFAULT_VERSION: &str = "latest";

/// The path to the package.json file relative to the worktree root.
const PACKAGE_JSON_PATH: &str = "package.json";

/// Represents a subset of package.json to extract dependency versions.
#[derive(Deserialize)]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, String>,
}

/// Detects the Angular version from the project's package.json.
pub fn detect_angular_version(worktree: &zed::Worktree) -> String {
    worktree
        .read_text_file(PACKAGE_JSON_PATH)
        .ok()
        .and_then(|content| serde_json::from_str::<PackageJson>(&content).ok())
        .and_then(|json| get_package_version(ANGULAR_CORE_PACKAGE, &json))
        .map(strip_semver_prefix)
        .unwrap_or_else(|| DEFAULT_VERSION.to_string())
}

/// Retrieves the version of a given package in a package.json.
fn get_package_version(package_name: &str, json: &PackageJson) -> Option<String> {
    json.dependencies.get(package_name)
        .or_else(|| json.dev_dependencies.get(package_name))
        .cloned()
}

/// Strip semver prefix characters from an npm version string.
fn strip_semver_prefix(version: String) -> String {
    version.trim_start_matches(|c| c == '^' || c == '~').to_string()
}
