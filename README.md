# Angular Language Server Zed Extension

[Angular](https://angular.dev) template support for [Zed](https://zed.dev),
powered by the [Angular Language Service](https://angular.dev/tools/language-service).

## Features

- Completions, diagnostics, hover info, and go-to-definition via the Angular Language Service
- Syntax highlighting for bindings, control flow, directives, and interpolations
- TypeScript highlighting and completions injected into binding expressions and interpolations
- Code outline listing elements, control flow blocks, defer blocks, and template variables
- Auto-indentation for HTML elements and control flow blocks
## Configuration

Add the following to your Zed `settings.json` to customize the extension:

```json
{
  "lsp": {
    "angular-language-server": {
      "initialization_options": {
        "force_strict_templates": true,
        "suppress_angular_diagnostic_codes": ["-998113"],
        "pin": {
          "@angular/language-server": "21.1.0",
          "typescript": "/absolute/path/to/typescript"
        }
      }
    }
  }
}
```

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `force_strict_templates` | `boolean` | `false` | Force-enables strict template type-checking, overriding your `tsconfig`. |
| `suppress_angular_diagnostic_codes` | `string[]` | `[]` | List of [Angular diagnostic codes](https://angular.dev/extended-diagnostics) to suppress, e.g. `["-998113"]`. The code for a diagnostic is shown in parentheses when hovering over it in the editor. |
| `pin` | `object` | `{}` | Pins specific packages to a version or local path, keyed by npm package name. Accepts a version string (e.g. `"21.1.0"`), `"latest"`, or an absolute path. Use with caution: incompatible combinations will prevent the language server from starting. Refer to the [Angular version compatibility matrix](https://angular.dev/reference/versions) before pinning. |

## Getting Started

1. Clone this repository
1. Add your changes
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

## Credits

This extension would not exist without the prior work done in
[nathansbradshaw/zed-angular](https://github.com/nathansbradshaw/zed-angular).
