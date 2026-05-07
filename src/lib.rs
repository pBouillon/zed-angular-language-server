use zed_extension_api as zed;

mod language_server;
mod package_manager;
mod semantic_version;

/// Represents the state of the Angular Language Server extension.
struct AngularLanguageServerExtension {
}

impl zed::Extension for AngularLanguageServerExtension {
    /// Initializes a new instance of the extension.
    ///
    /// Zed calls this exactly once when the extension is loaded into the
    /// editor.
    fn new() -> Self {
        Self { }
    }

    /// Defines the command required to start the Angular Language Server.
    ///
    /// Zed calls this method whenever a user opens a file associated with
    /// Angular, as defined in the `extension.toml`.
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let versions = package_manager::detect_project_versions(worktree);
        let binaries = language_server::resolve_binaries(language_server_id, &versions)?;

        Err(format!(
            "node: {} | server: {} | angular: {} | ts: {}",
            binaries.node, binaries.server_entry, versions.angular, versions.typescript
        ))
    }
}

// Registers the extension. This macro is required to expose our Rust code
// to Zed's WebAssembly host environment.
zed::register_extension!(AngularLanguageServerExtension);
