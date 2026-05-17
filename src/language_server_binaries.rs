use crate::extension_settings::ExtensionSettings;
use crate::log_info;
use crate::package_manager::AngularProjectVersions;
use crate::package_resolver::PackageResolver;
use crate::package_source::PackageSource;
use zed_extension_api as zed;

/// The name of the Angular Language Server npm package.
const ANGULAR_LANGUAGE_SERVER_PACKAGE: &str = "@angular/language-server";

/// The name of the TypeScript npm package.
const TYPESCRIPT_PACKAGE: &str = "typescript";

/// Resolved binary paths required to launch the Angular Language Server.
pub struct LanguageServerBinaries {
    /// The absolute path to the root directory of the intalled
    /// `@angular/language-server` package.
    pub angular_server_package_location: String,

    /// The absolute path to the Node.js binary provided by Zed.
    pub node: String,

    /// The absolute path to the root directory of the intalled `typescript`
    /// package.
    pub typescript_package_location: String,
}

impl LanguageServerBinaries {
    /// Resolves the paths for the Angular Language Server and TypeScript,
    /// taking into account any version or path pins in the extension settings.
    /// Pinned paths bypass npm resolution entirely; pinned versions override
    /// those inferred from the project's `package.json`.
    pub fn resolve(
        language_server_id: &zed::LanguageServerId,
        versions: &AngularProjectVersions,
        worktree: &zed::Worktree,
        settings: &ExtensionSettings,
    ) -> zed::Result<Self> {
        let package_resolver = PackageResolver::new(language_server_id, worktree)?;

        let angular_server_package_location = resolve_location(
            &package_resolver,
            ANGULAR_LANGUAGE_SERVER_PACKAGE,
            &versions.angular,
            settings
                .pin
                .get(ANGULAR_LANGUAGE_SERVER_PACKAGE)
                .map(String::as_str),
        )?;

        let typescript_package_location = resolve_location(
            &package_resolver,
            TYPESCRIPT_PACKAGE,
            &versions.typescript,
            settings.pin.get(TYPESCRIPT_PACKAGE).map(String::as_str),
        )?;

        Ok(Self {
            node: zed::node_binary_path()?,
            angular_server_package_location,
            typescript_package_location,
        })
    }
}

/// Resolves the location of a package, respecting any pin from the settings.
///
/// If the pin is a local path, it is returned directly without any npm
/// interaction. If it is a version string, that version is used instead of
/// the one inferred from the project. If no pin is set, the inferred version
/// is used.
fn resolve_location(
    resolver: &PackageResolver,
    package: &str,
    inferred_version: &str,
    pin: Option<&str>,
) -> zed::Result<String> {
    match pin.map(PackageSource::from_str) {
        Some(PackageSource::Path(path)) => {
            log_info!("Using pinned local path for {package}: {path}");
            Ok(path)
        }
        Some(PackageSource::Version(version)) => {
            log_info!(
                "Using pinned version for {package}: {version} (inferred: {inferred_version})"
            );
            resolver.resolve_package_location(package, &version)
        }
        None => resolver.resolve_package_location(package, inferred_version),
    }
}
