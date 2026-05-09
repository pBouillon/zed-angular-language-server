use zed_extension_api as zed;

use crate::package_manager::AngularProjectVersions;
use crate::package_resolver::PackageResolver;

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
    /// Installs the Angular Language Server and TypeScript npm packages
    /// and resolves their paths.
    pub fn resolve(
        language_server_id: &zed::LanguageServerId,
        versions: &AngularProjectVersions,
        worktree: &zed::Worktree,
    ) -> zed::Result<Self> {
        let package_resolver = PackageResolver::new(language_server_id, worktree)?;

        let angular_server_package_location = package_resolver
            .resolve_package_location(ANGULAR_LANGUAGE_SERVER_PACKAGE, &versions.angular)?;

        let typescript_package_location =
            package_resolver.resolve_package_location(TYPESCRIPT_PACKAGE, &versions.typescript)?;

        Ok(Self {
            node: zed::node_binary_path()?,
            angular_server_package_location,
            typescript_package_location,
        })
    }
}
