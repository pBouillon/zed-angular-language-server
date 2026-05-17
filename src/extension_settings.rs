use crate::log_info;
use serde::Deserialize;
use std::collections::HashMap;
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

    /// Pins specific npm packages to a version string or local path,
    /// keyed by their npm package name.
    ///
    /// Accepted values per entry:
    /// - A semantic version string, e.g. `"17.3.0"`
    /// - `"latest"`
    /// - An absolute path to a local package directory, e.g. `"/path/to/pkg"`
    #[serde(default)]
    pub pin: HashMap<String, String>,
}

impl ExtensionSettings {
    /// Reads the extension's LSP settings for the given worktree.
    /// Falls back to defaults when nothing is configured.
    pub fn for_worktree(id: &zed::LanguageServerId, worktree: &zed::Worktree) -> Self {
        let raw = zed::settings::LspSettings::for_worktree(id.as_ref(), worktree)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default_settings() {
        let settings = ExtensionSettings::default();

        assert_eq!(settings.force_strict_templates, None);
        assert!(settings.suppress_angular_diagnostic_codes.is_empty());
    }

    #[test]
    fn test_deserialize_empty_json() {
        let json_data = json!({});
        let settings: ExtensionSettings = serde_json::from_value(json_data).unwrap();

        assert_eq!(settings.force_strict_templates, None);
        assert!(settings.suppress_angular_diagnostic_codes.is_empty());
    }

    #[test]
    fn test_deserialize_full_json() {
        let json_data = json!({
            "force_strict_templates": true,
            "suppress_angular_diagnostic_codes": ["-998114", "-998101"]
        });

        let settings: ExtensionSettings = serde_json::from_value(json_data).unwrap();

        assert_eq!(settings.force_strict_templates, Some(true));
        assert_eq!(
            settings.suppress_angular_diagnostic_codes,
            vec!["-998114".to_string(), "-998101".to_string()]
        );
    }

    #[test]
    fn test_deserialize_partial_json_only_strict() {
        let json_data = json!({
            "force_strict_templates": false
        });

        let settings: ExtensionSettings = serde_json::from_value(json_data).unwrap();

        assert_eq!(settings.force_strict_templates, Some(false));
        assert!(settings.suppress_angular_diagnostic_codes.is_empty());
    }

    #[test]
    fn test_deserialize_partial_json_only_suppress() {
        let json_data = json!({
            "suppress_angular_diagnostic_codes": ["8002"]
        });

        let settings: ExtensionSettings = serde_json::from_value(json_data).unwrap();

        assert_eq!(settings.force_strict_templates, None);
        assert_eq!(
            settings.suppress_angular_diagnostic_codes,
            vec!["8002".to_string()]
        );
    }

    #[test]
    fn test_deserialize_invalid_types_fallback() {
        let json_data = json!({
            "force_strict_templates": "true"
        });

        let result: Result<ExtensionSettings, _> = serde_json::from_value(json_data);
        assert!(
            result.is_err(),
            "Expected an error when parsing invalid types"
        );
    }
}
