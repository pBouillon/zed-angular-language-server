## Changelog

### v1.1.0 (unreleased)

- feat: add TypeScript and Angular Language Server pinning option in the settings.

### v1.0.0 (2026-05-10)

#### Language

- feat: syntax highlighting for Angular templates
- feat: TypeScript highlighting and completions injected into binding expressions
- feat: code outline for elements, control flow blocks, defer blocks, and template variables

#### Extension

- feat: Angular Language Service integration
- feat: automatic Angular and TypeScript version detection from `package.json`
- feat: TypeScript version fallback based on Angular version compatibility matrix
- feat: `force_strict_templates` setting to override tsconfig strict template checking
- feat: `suppress_angular_diagnostic_codes` setting to silence specific Angular diagnostics
