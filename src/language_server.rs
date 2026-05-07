use zed_extension_api::{self as zed};

use crate::language_server_binaries::LanguageServerBinaries;

/// Represents a resolved Angular Language Server instance, ready to be
/// launched by Zed.
pub struct AngularLanguageServer {
    binaries: LanguageServerBinaries,
}

impl AngularLanguageServer {
    /// Returns the [`zed::Command`] required to start the Angular Language
    /// Server.
    pub fn command(&self) -> zed::Command {
        zed::Command {
            command: self.binaries.node.clone(),
            args: vec![
                self.binaries.server_entry.clone(),
                "--stdio".to_string(),
                "--tsProbeLocations".to_string(),
                self.binaries.node_modules.clone(),
                "--ngProbeLocations".to_string(),
                self.binaries.node_modules.clone(),
            ],
            env: Default::default(),
        }
    }
}

impl From<LanguageServerBinaries> for AngularLanguageServer {
    fn from(binaries: LanguageServerBinaries) -> Self {
        Self { binaries }
    }
}
