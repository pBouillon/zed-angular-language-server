use zed_extension_api as zed;

/// Represents the state of the Angular Language Server extension.
struct AngularLanguageServerExtension {
}

impl AngularLanguageServerExtension {
}

impl zed::Extension for AngularLanguageServerExtension {
    /// Initializes a new instance of the extension.
    ///
    /// Zed calls this exactly once when the
    /// extension is loaded into the editor.
    fn new() -> Self {
        Self { }
    }

    /// Defines the command required to start the Angular Language Server.
    ///
    /// Zed calls this method whenever a user opens a file associated with
    /// Angular, as defined in the `extension.toml`.
    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        todo!()
    }
}

// Registers the extension. This macro is required to expose our Rust code
// to Zed's WebAssembly host environment.
zed::register_extension!(AngularLanguageServerExtension);
