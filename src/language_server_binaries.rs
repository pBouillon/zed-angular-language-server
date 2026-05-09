use std::env;
use zed_extension_api::{self as zed, LanguageServerInstallationStatus};

use crate::package_manager::AngularProjectVersions;
use crate::{log_error, log_info};

/// The name of the Angular Language Server npm package.
const ANGULAR_LANGUAGE_SERVER_PACKAGE: &str = "@angular/language-server";

/// The path to the Angular Language Server entry point, relative to the
/// extension's working directory.
const ANGULAR_LANGUAGE_SERVER_PATH: &str = "node_modules/@angular/language-server/index.js";

/// The name of the TypeScript npm package.
const TYPESCRIPT_PACKAGE: &str = "typescript";

/// The path to the TypeScript lib, relative to the extension's working directory.
const TYPESCRIPT_LIB_PATH: &str = "node_modules/typescript/lib";

/// Resolved binary paths required to launch the Angular Language Server.
pub struct LanguageServerBinaries {
    pub angular_server_entrypoint: String,
    pub angular_server_package_location: String,
    pub node: String,
    pub typescript_entrypoint: String,
    pub typescript_package_location: String,
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

        zed::set_language_server_installation_status(id, &LanguageServerInstallationStatus::None);

        result
    }

    /// Installs the Angular Language Server and TypeScript npm packages into the
    /// extension directory and resolves their paths.
    pub fn resolve(
        language_server_id: &zed::LanguageServerId,
        versions: &AngularProjectVersions,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Self> {
        let zed_node = zed::node_binary_path()?;

        if !is_package_present_in_version(ANGULAR_LANGUAGE_SERVER_PACKAGE, &versions.angular) {
            Self::download_package(
                language_server_id,
                ANGULAR_LANGUAGE_SERVER_PACKAGE,
                &versions.angular,
            )?;
        }

        if !is_package_present_in_version(TYPESCRIPT_PACKAGE, &versions.typescript) {
            Self::download_package(language_server_id, TYPESCRIPT_PACKAGE, &versions.typescript)?;
        }

        let extension_dir = env::current_dir().map_err(|error| error.to_string())?;
        Ok(Self {
            node: zed_node,
            angular_server_entrypoint: resolve_path_to(
                &extension_dir,
                ANGULAR_LANGUAGE_SERVER_PATH,
            ),
            angular_server_package_location: resolve_path_to(
                &extension_dir,
                ANGULAR_LANGUAGE_SERVER_PACKAGE,
            ),
            typescript_package_location: resolve_path_to(&extension_dir, TYPESCRIPT_PACKAGE),
            typescript_entrypoint: resolve_path_to(&extension_dir, TYPESCRIPT_LIB_PATH),
        })
    }
}

/// Returns `true` if the given npm package is already installed in the extension
/// at exactly the required version.
fn is_package_present_in_version(package: &str, required: &str) -> bool {
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
