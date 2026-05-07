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

/// The path to the TypeScript package directory, relative to the extension's
/// working directory. The Angular Language Server uses this as a probe
/// location to find the TypeScript installation.
const TYPESCRIPT_PATH: &str = "node_modules/typescript";

/// Resolved binary paths required to launch the Angular Language Server.
pub struct LanguageServerBinaries {
    /// Path to the Node.js binary used to run the server.
    pub node: String,
    /// Absolute path to the Angular Language Server's main script.
    pub server_entry: String,
    /// Absolute path to the TypeScript package directory.
    pub typescript_path: String,
}

/// Installs the Angular Language Server and TypeScript npm packages if
/// necessary and resolves the paths to the binaries required to run them.
///
/// The Angular Language Server version is pinned to the Angular version
/// detected in the project, as the two packages are always released together
/// with matching versions.
pub fn resolve_binaries(
    language_server_id: &zed::LanguageServerId,
    versions: &AngularProjectVersions,
) -> zed::Result<LanguageServerBinaries> {
    let node = zed::node_binary_path()?;

    zed::set_language_server_installation_status(
        language_server_id,
        &LanguageServerInstallationStatus::Downloading,
    );

    let install_result = zed::npm_install_package(ANGULAR_LANGUAGE_SERVER_PACKAGE, &versions.angular)
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

    Ok(LanguageServerBinaries {
        node,
        server_entry: ANGULAR_LANGUAGE_SERVER_PATH.to_string(),
        typescript_path: TYPESCRIPT_PATH.to_string(),
    })
}
