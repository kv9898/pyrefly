# Selective Language Service Disabling

Pyrefly's LSP server supports selective disabling of individual language services via command-line arguments. This allows users to choose exactly which IDE features they want enabled, providing flexibility for different workflows and preferences.

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

### Command-Line Arguments

When starting the LSP server, you can use command-line flags to disable specific services:

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

### Configuring via Editor Settings

For editors that support passing arguments to the LSP server (like VSCode), you can configure the arguments through editor settings. For VSCode, use the `pyrefly.lspArguments` setting:

```json
{
  "pyrefly.lspArguments": [
    "lsp",
    "--disable-hover",
    "--disable-document-symbol"
  ]
}
```

This allows you to configure which services are disabled without manually starting the LSP server.

## Implementation Details

### Architecture Decision

The selective disabling is implemented at the **binary level** (LSP server) via command-line arguments. This design provides several benefits:

1. **Cross-editor compatibility** - Works with any editor that uses Pyrefly's LSP server
2. **Cleaner architecture** - The server respects capabilities through standard LSP initialization
3. **Performance** - Disabled services are not advertised as capabilities
4. **Maintainability** - Logic is centralized in one place

### How It Works

1. **Initialization**: Disabled services are passed via command-line arguments (stored in `LspArgs`)

2. **Capability Negotiation**: The server's `capabilities` function checks disabled services and omits them from the advertised capabilities during LSP initialization.

3. **Service Availability**: Once capabilities are negotiated, clients will not request disabled services since they are not advertised.

## Use Cases

### Scenario 1: Performance Optimization

If you're working on a very large codebase and find that certain language services are slow, you can disable them to improve performance:

```json
{
  "pyrefly.lspArguments": [
    "lsp",
    "--disable-references",
    "--disable-workspace-symbol"
  ]
}
```

### Scenario 2: Conflict Resolution

If you're using multiple language servers and want to use Pyrefly for type checking but another server for certain features:

```json
{
  "pyrefly.lspArguments": [
    "lsp",
    "--disable-completion",
    "--disable-hover"
  ]
}
```

### Scenario 3: Minimal Mode

For a minimal setup with only type checking and go-to-definition:

```json
{
  "pyrefly.lspArguments": [
    "lsp",
    "--disable-hover",
    "--disable-completion",
    "--disable-signature-help",
    "--disable-document-highlight",
    "--disable-document-symbol",
    "--disable-workspace-symbol",
    "--disable-semantic-tokens",
    "--disable-inlay-hint"
  ]
}
```

## Testing

The implementation includes tests to ensure:
- Command-line arguments are correctly parsed
- Disabled services are omitted from capabilities
- The LSP server functions correctly with services disabled

Run tests with:
```bash
cargo test --package pyrefly --lib
```
