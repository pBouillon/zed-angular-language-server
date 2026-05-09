use zed_extension_api as zed;

/// Builds the code label for the editor completion suggestions from the LSP completion.
pub fn build_label(completion: zed::lsp::Completion) -> Option<zed::CodeLabel> {
    let highlight = to_syntax_category(completion.kind?)?;
    let filter_range = 0..completion.label.len();

    let (detail, description) = completion
        .label_details
        .map(|d| (d.detail, d.description))
        .unwrap_or_default();

    let spans = [
        Some(extract_primary_identifier(completion.label, highlight)),
        detail.map(call_signature),
        description.map(source_origin),
        completion.detail.map(type_hint),
    ]
    .into_iter()
    .flatten()
    .flatten()
    .collect();

    Some(zed::CodeLabel {
        code: Default::default(),
        spans,
        filter_range: filter_range.into(),
    })
}

/// Creates the core identifier portion of the label.
fn extract_primary_identifier(label: String, highlight: &'static str) -> Vec<zed::CodeLabelSpan> {
    vec![zed::CodeLabelSpan::literal(
        label,
        Some(highlight.to_string()),
    )]
}

/// Formats the functional signature or parameter list.
fn call_signature(text: String) -> Vec<zed::CodeLabelSpan> {
    vec![zed::CodeLabelSpan::literal(
        text,
        Some("punctuation.special".to_string()),
    )]
}

/// Formats the origin or namespace context of the symbol.
fn source_origin(text: String) -> Vec<zed::CodeLabelSpan> {
    vec![
        zed::CodeLabelSpan::literal(" ".to_string(), None),
        zed::CodeLabelSpan::literal(text, Some("comment".to_string())),
    ]
}

/// Formats supplementary type information or hints.
fn type_hint(text: String) -> Vec<zed::CodeLabelSpan> {
    vec![
        zed::CodeLabelSpan::literal(" — ", None),
        zed::CodeLabelSpan::literal(text, Some("hint".to_string())),
    ]
}

/// Maps completion categories to theme-specific syntax highlighting tokens.
fn to_syntax_category(kind: zed::lsp::CompletionKind) -> Option<&'static str> {
    use zed::lsp::CompletionKind as Kind;
    match kind {
        Kind::Class | Kind::Interface => Some("type"),
        Kind::Constructor => Some("constructor"),
        Kind::Constant => Some("constant"),
        Kind::Function | Kind::Method => Some("function"),
        Kind::Property | Kind::Field => Some("property"),
        Kind::Variable => Some("variable"),
        Kind::Keyword => Some("keyword"),
        Kind::Enum => Some("enum"),
        Kind::Module => Some("module"),
        _ => None,
    }
}
