use crate::ExtensionSettings;
use crate::language_server_binaries::LanguageServerBinaries;
use std::path::PathBuf;
use std::{env, vec};
use zed_extension_api::{self as zed, Result};

/// Represents a resolved Angular Language Server instance, ready to be
/// launched by Zed.
pub struct AngularLanguageServer {
    binaries: LanguageServerBinaries,
}

impl AngularLanguageServer {
    /// Returns the [`zed::Command`] required to start the Angular Language
    /// Server.
    pub fn command(
        &self,
        worktree: Option<&zed::Worktree>,
        settings: &ExtensionSettings,
    ) -> zed::Command {
        let mut args = vec![
            self.binaries.angular_server_package_location.clone(),
            "--stdio".to_string(),
        ];

        // Resolve TypeScript probe locations
        args.push("--tsProbeLocations".to_string());
        args.extend(
            self.resolve_probe_locations(&self.binaries.typescript_package_location, worktree),
        );

        // Resolve Angular probe locations
        args.push("--ngProbeLocations".to_string());
        args.extend(
            self.resolve_probe_locations(&self.binaries.angular_server_package_location, worktree),
        );

        args.push("--tsdk".to_string());
        args.push(self.binaries.typescript_package_location.clone());

        // User settings
        match settings.force_strict_templates {
            Some(true) => args.push("--forceStrictTemplates".to_string()),
            Some(false) | None => {}
        }

        if !settings.suppress_angular_diagnostic_codes.is_empty() {
            args.push("--suppressAngularDiagnosticCodes".to_string());
            args.push(settings.suppress_angular_diagnostic_codes.join(","));
        }

        zed::Command {
            command: self.binaries.node.clone(),
            args,
            env: Default::default(),
        }
    }

    /// Consolidates probe locations for the language server flags.
    /// Includes the package's internal location, the extension's current directory,
    /// and the user's project worktree root.
    fn resolve_probe_locations(
        &self,
        package_location: &str,
        worktree: Option<&zed::Worktree>,
    ) -> Vec<String> {
        let mut locations = vec![package_location.to_string()];

        if let Ok(current_dir) = get_current_directory() {
            locations.push(current_dir.to_string_lossy().to_string());
        }

        if let Some(worktree) = worktree {
            locations.push(worktree.root_path());
        }

        locations
    }
}

impl From<LanguageServerBinaries> for AngularLanguageServer {
    fn from(binaries: LanguageServerBinaries) -> Self {
        Self { binaries }
    }
}

/// Attempts to get the process's current directory
fn get_current_directory() -> Result<PathBuf> {
    env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))
}
