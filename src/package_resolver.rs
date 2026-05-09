use std::env;
use std::path::PathBuf;
use zed_extension_api::{self as zed, LanguageServerInstallationStatus};

use crate::{log_error, log_info};

/// The name of the directory in which Node.js dependencies are downloaded.
const NODE_MODULES: &str = "node_modules";

/// A coordinator responsible for locating and preparing the Node.js packages
/// required by the extension.
pub struct PackageResolver<'a> {
    /// A reference to the unique identifier for the language server instance
    /// is used to report installation progress and errors back to the editor.
    language_server_id: &'a zed::LanguageServerId,

    /// The absolute path to the root of the current project worktree used
    /// to look up project-local `node_modules`.
    worktree_root: PathBuf,

    /// The absolute path to the directory where the extension is installed
    /// used as a fallback location to install packages
    extension_dir: PathBuf,
}

impl<'a> PackageResolver<'a> {
    /// Creates a new instance of the resolver by capturing the current environment.
    pub fn new(id: &'a zed::LanguageServerId, worktree: &zed::Worktree) -> zed::Result<Self> {
        Ok(Self {
            language_server_id: id,
            worktree_root: PathBuf::from(worktree.root_path()),
            extension_dir: env::current_dir().map_err(|e| e.to_string())?,
        })
    }

    /// Locates the specified package by checking the project's `node_modules`
    /// first, falling back to a version-checked installation in the extension
    /// directory.
    pub fn resolve_package_location(&self, name: &str, version: &str) -> zed::Result<String> {
        let local_path = self.worktree_root.join(NODE_MODULES).join(name);

        if local_path.exists() {
            log_info!("Using project-local {name}");
            return Ok(local_path.to_string_lossy().to_string());
        }

        if !is_package_present_in_version(name, version) {
            self.download_in_version(name, version)?;
        }

        let package_location = self
            .extension_dir
            .join(NODE_MODULES)
            .join(name)
            .to_string_lossy()
            .to_string();

        Ok(package_location)
    }

    /// Handles the physical installation of an NPM package and reports the
    /// installation status back to the editor.
    fn download_in_version(&self, package: &str, version: &str) -> zed::Result<()> {
        log_info!("Installing package: {package}@{version}");

        zed::set_language_server_installation_status(
            self.language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let result = zed::npm_install_package(package, version);

        if let Err(ref error) = result {
            log_error!("Installation of {package} failed: {error}");
            zed::set_language_server_installation_status(
                self.language_server_id,
                &LanguageServerInstallationStatus::Failed(error.clone()),
            );
        } else {
            zed::set_language_server_installation_status(
                self.language_server_id,
                &LanguageServerInstallationStatus::None,
            );
        }

        result
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
