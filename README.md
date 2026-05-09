# Angular Language Server — Zed Extension

[Angular](https://angular.dev) template support for [Zed](https://zed.dev),
powered by the [Angular Language Service](https://angular.dev/tools/language-service).

## Features

- Completions, diagnostics, hover info, and go-to-definition via the Angular Language Service
- Syntax highlighting for bindings, control flow, directives, and interpolations
- TypeScript highlighting and completions injected into binding expressions and interpolations
- Code outline listing elements, control flow blocks, defer blocks, and template variables
- Auto-indentation for HTML elements and control flow blocks

## Getting Started

1. Clone this repository
1. Add your changes
1. Build the extension for WASM by running `cargo build-extension`
1. Open Zed, navigate to the extension window and click on `Install Dev Extension`
1. Select the folder in which is placed the `extension.toml`

## Contributing

Pull requests are welcome. For significant changes, open a discussion first to
align on the approach. Please ensure `cargo fmt` and `cargo clippy` pass before
submitting.

## Reporting a Bug

Open a [GitHub issue](https://github.com/pbouillon/zed-angular-language-server/issues) and include:

- A minimal template that reproduces the problem
- What you expected vs. what happened
- If you're willing to help fixing it

## Feature Requests

Open a [GitHub issue](https://github.com/pbouillon/zed-angular-language-server/issues) describing the use case, or submit a pull
request directly.
