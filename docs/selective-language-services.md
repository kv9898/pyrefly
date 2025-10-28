# Selective Language Service Disabling

Pyrefly's LSP server now supports selective disabling of individual language services. This allows users to choose exactly which IDE features they want enabled, providing flexibility for different workflows and preferences.

## Available Language Services

The following language services can be selectively disabled:

1. **definition** - Go-to-definition
2. **typeDefinition** - Go-to-type-definition
3. **codeAction** - Code actions (quick fixes)
4. **completion** - Autocomplete
5. **documentHighlight** - Highlight references
6. **references** - Find references
7. **rename** - Rename symbol
8. **signatureHelp** - Parameter hints
9. **hover** - Hover tooltips
10. **inlayHint** - Inlay hints
11. **documentSymbol** - Document symbols (outline)
12. **workspaceSymbol** - Workspace-wide symbol search
13. **semanticTokens** - Semantic highlighting

## Usage

### VSCode Extension

Add the following to your VSCode settings (`.vscode/settings.json` or user settings):

```json
{
  "python.pyrefly.disabledLanguageServices": {
    "hover": true,
    "documentSymbol": true
  }
}
```

This example disables hover tooltips and document symbols (outline), while keeping all other services enabled.

### Command-Line Arguments

When starting the LSP server manually, you can use command-line flags:

```bash
pyrefly lsp --disable-hover --disable-document-symbol
```

Available flags:
- `--disable-definition`
- `--disable-type-definition`
- `--disable-code-action`
- `--disable-completion`
- `--disable-document-highlight`
- `--disable-references`
- `--disable-rename`
- `--disable-signature-help`
- `--disable-hover`
- `--disable-inlay-hint`
- `--disable-document-symbol`
- `--disable-workspace-symbol`
- `--disable-semantic-tokens`

### Passing Arguments via VSCode Extension

You can also pass these command-line arguments through the VSCode extension settings:

```json
{
  "pyrefly.lspArguments": [
    "lsp",
    "--disable-hover",
    "--disable-document-symbol"
  ]
}
```

## Implementation Details

### Architecture Decision

The selective disabling is implemented at the **binary level** (LSP server) rather than the VSCode extension level. This design decision provides several benefits:

1. **Cross-editor compatibility** - Works with any editor that uses Pyrefly's LSP server
2. **Cleaner architecture** - The server respects capabilities rather than the client filtering
3. **Performance** - Server doesn't process requests for disabled services
4. **Maintainability** - Logic is centralized in one place

### How It Works

1. **Initialization**: Disabled services are communicated to the server via:
   - Command-line arguments (stored in `LspArgs`)
   - IDE configuration (sent via `workspace/configuration` request)

2. **Capability Negotiation**: The server's `capabilities` function checks disabled services and omits them from the advertised capabilities during LSP initialization.

3. **Runtime Checks**: For services that are disabled via IDE configuration after initialization, the server performs runtime checks before processing requests.

## Use Cases

### Scenario 1: Performance Optimization

If you're working on a very large codebase and find that certain language services are slow, you can disable them to improve performance:

```json
{
  "python.pyrefly.disabledLanguageServices": {
    "references": true,
    "workspaceSymbol": true
  }
}
```

### Scenario 2: Conflict Resolution

If you're using multiple language servers and want to use Pyrefly for type checking but another server for certain features:

```json
{
  "python.pyrefly.disabledLanguageServices": {
    "completion": true,
    "hover": true
  }
}
```

### Scenario 3: Minimal Mode

For a minimal setup with only type checking and go-to-definition:

```json
{
  "python.pyrefly.disabledLanguageServices": {
    "hover": true,
    "completion": true,
    "signatureHelp": true,
    "documentHighlight": true,
    "documentSymbol": true,
    "workspaceSymbol": true,
    "semanticTokens": true,
    "inlayHint": true
  }
}
```

## Backward Compatibility

The existing `python.pyrefly.disableLanguageServices` boolean setting still works and will disable **all** language services when set to `true`. The new selective disabling is fully backward compatible.

## Testing

The implementation includes comprehensive tests to ensure:
- Individual services can be disabled independently
- Other services continue to work when one is disabled
- Configuration changes are applied correctly
- Command-line arguments work as expected

Run tests with:
```bash
cargo test --package pyrefly --lib test::lsp::lsp_interaction::configuration
```
