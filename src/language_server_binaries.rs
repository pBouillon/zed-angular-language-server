use zed_extension_api::{self as zed, LanguageServerInstallationStatus};

use crate::package_manager::AngularProjectVersions;
use crate::{log_error, log_info};

/// The name of the Angular Language Server npm package.
const ANGULAR_LANGUAGE_SERVER_PACKAGE: &str = "@angular/language-server";

/// The path to the Angular Language Server entry point, relative to the
/// extension's working directory (where npm packages are installed).
const ANGULAR_LANGUAGE_SERVER_PATH: &str = "node_modules/@angular/language-server/index.js";

/// The name of the TypeScript npm package.
const TYPESCRIPT_PACKAGE: &str = "typescript";

/// The directory in which npm packages are installed, relative to the
/// extension's working directory. Used as the probe location for both
/// `--tsProbeLocations` and `--ngProbeLocations` when starting the server.
const NODE_MODULES_PATH: &str = "node_modules";

/// Resolved binary paths required to launch the Angular Language Server.
pub struct LanguageServerBinaries {
    pub node: String,
    pub server_entry: String,
    pub node_modules: String,
}

impl LanguageServerBinaries {
    /// Performs the NPM installation for a specific package and manages
    /// the UI installation status.
    fn download_package(
        id: &zed::LanguageServerId,
        package: &str,
        version: &str,
    ) -> zed::Result<()> {
        log_info!("Installing package: {package}@{version}");

        zed::set_language_server_installation_status(
            id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let result = zed::npm_install_package(package, version);

        if let Err(ref error) = result {
            log_error!("Installation of {package} failed: {error}");
            zed::set_language_server_installation_status(
                id,
                &LanguageServerInstallationStatus::Failed(error.clone()),
            );
        }

        result
    }

    /// Checks if a package is at the required version.
    /// Logs if it is, otherwise triggers a download.
    fn ensure_package_installed_in_version(
        id: &zed::LanguageServerId,
        package: &str,
        version: &str,
    ) -> zed::Result<()> {
        if is_package_at_version(package, version) {
            log_info!("{package}@{version} already up to date, skipping install.");
            return Ok(());
        }

        Self::download_package(id, package, version)
    }

    /// Installs the Angular Language Server and TypeScript npm packages if
    /// necessary and resolves the paths required to run the server.
    ///
    /// The Angular Language Server version is pinned to the Angular version
    /// detected in the project, as the two packages are always released
    /// together with matching versions.
    pub fn resolve(
        language_server_id: &zed::LanguageServerId,
        versions: &AngularProjectVersions,
    ) -> zed::Result<Self> {
        let node = zed::node_binary_path()?;

        Self::ensure_package_installed_in_version(
            language_server_id,
            ANGULAR_LANGUAGE_SERVER_PACKAGE,
            &versions.angular,
        )?;
        Self::ensure_package_installed_in_version(
            language_server_id,
            TYPESCRIPT_PACKAGE,
            &versions.typescript,
        )?;

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        let extension_dir = std::env::current_dir().map_err(|error| error.to_string())?;

        Ok(Self {
            node,
            server_entry: resolve_path_to(&extension_dir, ANGULAR_LANGUAGE_SERVER_PATH),
            node_modules: resolve_path_to(&extension_dir, NODE_MODULES_PATH),
        })
    }
}

/// Returns `true` if the given npm package is already installed at exactly the
/// required version, `false` in any other case (not installed, different
/// version, or an error querying the installed version).
fn is_package_at_version(package: &str, required: &str) -> bool {
    zed::npm_package_installed_version(package)
        .ok()
        .flatten()
        .map(|installed| {
            let matches = installed == required;
            if !matches {
                log_info!(
                    "{package}: installed={installed}, required={required} — will reinstall."
                );
            }
            matches
        })
        .unwrap_or(false)
}

/// Joins a base path with a relative subpath and converts it to a lossy String.
fn resolve_path_to(base: &std::path::Path, subpath: &str) -> String {
    base.join(subpath).to_string_lossy().to_string()
}
