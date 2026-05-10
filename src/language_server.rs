use crate::ExtensionSettings;
use crate::language_server_binaries::LanguageServerBinaries;
use std::path::PathBuf;
use std::{env, vec};
use zed_extension_api::{self as zed, Result};

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
        let ts_probe_locations =
            self.resolve_probe_locations(&self.binaries.typescript_package_location, worktree);
        let ng_probe_locations =
            self.resolve_probe_locations(&self.binaries.angular_server_package_location, worktree);

        zed::Command {
            command: self.binaries.node.clone(),
            args: build_args(
                &self.binaries.angular_server_package_location,
                &self.binaries.typescript_package_location,
                &ts_probe_locations,
                &ng_probe_locations,
                settings,
            ),
            env: Default::default(),
        }
    }

    /// Consolidates probe locations for the language server flags.
    /// If resolved, includes the package's internal location, the extension's
    /// current directory, and the user's project worktree root.
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

/// Constructs the argument list for the Angular Language Server process from
/// resolved binary locations and user settings.
fn build_args(
    angular_server_location: &str,
    typescript_location: &str,
    ts_probe_locations: &[String],
    ng_probe_locations: &[String],
    settings: &ExtensionSettings,
) -> Vec<String> {
    let mut args = vec![angular_server_location.to_string(), "--stdio".to_string()];

    args.push("--tsProbeLocations".to_string());
    args.extend_from_slice(ts_probe_locations);

    args.push("--ngProbeLocations".to_string());
    args.extend_from_slice(ng_probe_locations);

    args.push("--tsdk".to_string());
    args.push(typescript_location.to_string());

    if settings.force_strict_templates == Some(true) {
        args.push("--forceStrictTemplates".to_string());
    }

    if !settings.suppress_angular_diagnostic_codes.is_empty() {
        args.push("--suppressAngularDiagnosticCodes".to_string());
        args.push(settings.suppress_angular_diagnostic_codes.join(","));
    }

    args
}

/// Attempts to get the process's current directory.
fn get_current_directory() -> Result<PathBuf> {
    env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(settings: &ExtensionSettings) -> Vec<String> {
        build_args(
            "/angular/language/server/location",
            "/typescript/location",
            &["/typescript/location".to_string()],
            &["/angular/language/server/location".to_string()],
            settings,
        )
    }

    #[test]
    fn test_full_command_line() {
        let settings = ExtensionSettings {
            force_strict_templates: Some(true),
            suppress_angular_diagnostic_codes: vec!["-998114".to_string(), "-998101".to_string()],
        };

        assert_eq!(
            build(&settings),
            vec![
                "/angular/language/server/location",
                "--stdio",
                "--tsProbeLocations",
                "/typescript/location",
                "--ngProbeLocations",
                "/angular/language/server/location",
                "--tsdk",
                "/typescript/location",
                "--forceStrictTemplates",
                "--suppressAngularDiagnosticCodes",
                "-998114,-998101",
            ]
        );
    }

    #[test]
    fn test_includes_required_flags() {
        let args = build(&ExtensionSettings::default());
        assert!(args.contains(&"--stdio".to_string()));
        assert!(args.contains(&"--tsProbeLocations".to_string()));
        assert!(args.contains(&"--ngProbeLocations".to_string()));
        assert!(args.contains(&"--tsdk".to_string()));
    }

    #[test]
    fn test_angular_server_location_is_first_arg() {
        let args = build(&ExtensionSettings::default());
        assert_eq!(args[0], "/angular/language/server/location");
    }

    #[test]
    fn test_tsdk_is_followed_by_typescript_location() {
        let args = build(&ExtensionSettings::default());
        let tsdk_index = args.iter().position(|a| a == "--tsdk").unwrap();
        assert_eq!(args[tsdk_index + 1], "/typescript/location");
    }

    #[test]
    fn test_force_strict_templates_absent_by_default() {
        let args = build(&ExtensionSettings::default());
        assert!(!args.contains(&"--forceStrictTemplates".to_string()));
    }

    #[test]
    fn test_force_strict_templates_added_when_enabled() {
        let settings = ExtensionSettings {
            force_strict_templates: Some(true),
            ..Default::default()
        };
        assert!(build(&settings).contains(&"--forceStrictTemplates".to_string()));
    }

    #[test]
    fn test_force_strict_templates_absent_when_disabled() {
        let settings = ExtensionSettings {
            force_strict_templates: Some(false),
            ..Default::default()
        };
        assert!(!build(&settings).contains(&"--forceStrictTemplates".to_string()));
    }

    #[test]
    fn test_suppress_diagnostic_codes_absent_by_default() {
        let args = build(&ExtensionSettings::default());
        assert!(!args.contains(&"--suppressAngularDiagnosticCodes".to_string()));
    }

    #[test]
    fn test_suppress_diagnostic_codes_joined_as_single_arg() {
        let settings = ExtensionSettings {
            suppress_angular_diagnostic_codes: vec!["-998114".to_string(), "-998101".to_string()],
            ..Default::default()
        };
        let args = build(&settings);
        let flag_index = args
            .iter()
            .position(|a| a == "--suppressAngularDiagnosticCodes")
            .unwrap();
        assert_eq!(args[flag_index + 1], "-998114,-998101");
    }
}
