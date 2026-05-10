use crate::log_info;
use serde::Deserialize;
use zed_extension_api::{self as zed};

#[derive(Debug, Deserialize, Default)]
pub struct ExtensionSettings {
    /// Controls Angular's strict template type-checking mode:
    /// - `true` → enable strict templates, overriding tsconfig
    /// - `false` or Omitted → defer to the tsconfig setting
    #[serde(default)]
    pub force_strict_templates: Option<bool>,

    /// Comma-separated list of TypeScript/Angular diagnostic codes to suppress,
    /// e.g. `"2003,2345"`.
    #[serde(default)]
    pub suppress_angular_diagnostic_codes: Vec<String>,
}

impl ExtensionSettings {
    /// Reads the extension's LSP settings for the given worktree.
    /// Falls back to defaults when nothing is configured.
    pub fn for_worktree(id: &zed::LanguageServerId, worktree: &zed::Worktree) -> Self {
        let raw = zed::settings::LspSettings::for_worktree(&id.to_string(), worktree)
            .ok()
            .and_then(|lsp| lsp.initialization_options);

        log_info!(
            "loaded settings: {}",
            match &raw {
                Some(value) => value.to_string(),
                None => "none".to_string(),
            }
        );

        raw.and_then(|value| serde_json::from_value(value).ok())
            .unwrap_or_default()
    }
}
