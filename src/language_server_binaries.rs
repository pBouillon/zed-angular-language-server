use zed_extension_api::{self as zed, LanguageServerInstallationStatus};

use crate::package_manager::AngularProjectVersions;

/// The name of the Angular Language Server npm package.
const ANGULAR_LANGUAGE_SERVER_PACKAGE: &str = "@angular/language-server";

/// The path to the Angular Language Server entry point, relative to the
/// extension's working directory (where npm packages are installed).
const ANGULAR_LANGUAGE_SERVER_PATH: &str =
    "node_modules/@angular/language-server/index.js";

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

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let install_result =
            zed::npm_install_package(ANGULAR_LANGUAGE_SERVER_PACKAGE, &versions.angular)
                .and_then(|_| zed::npm_install_package(TYPESCRIPT_PACKAGE, &versions.typescript));

        if let Err(ref error) = install_result {
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Failed(error.clone()),
            );
        }

        install_result?;

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        let extension_dir = std::env::current_dir().map_err(|e| e.to_string())?;

        Ok(Self {
            node,
            server_entry: extension_dir.join(ANGULAR_LANGUAGE_SERVER_PATH).to_string_lossy().to_string(),
            node_modules: extension_dir.join(NODE_MODULES_PATH).to_string_lossy().to_string(),
        })
    }
}
