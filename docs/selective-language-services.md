# Selective Language Service Disabling

Pyrefly's LSP server supports selective disabling of individual language services via workspace configuration. This allows users to choose exactly which IDE features they want enabled, providing flexibility for different workflows and preferences.

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

### Workspace Configuration

You can configure which language services to disable through your editor's settings or workspace configuration. The configuration is part of the LSP Analysis settings:

#### VSCode

Add the following to your `settings.json` (workspace or user settings):

```json
{
  "pyrefly.analysis.disabledLanguageServices": {
    "hover": true,
    "documentSymbol": true
  }
}
```

#### General LSP Configuration

For other editors that support LSP workspace configuration, send a configuration with the following structure:

```json
{
  "pyrefly": {
    "analysis": {
      "disabledLanguageServices": {
        "hover": true,
        "completion": true,
        "signatureHelp": true
      }
    }
  }
}
```

## Implementation Details

### Architecture Decision

The selective disabling is implemented at the **workspace configuration level**. This design provides several benefits:

1. **Dynamic Configuration** - Services can be disabled/enabled without restarting the LSP server
2. **Per-workspace Control** - Different workspace folders can have different service configurations
3. **Editor Integration** - Works seamlessly with editor configuration systems (VSCode settings, etc.)
4. **Standard LSP Pattern** - Uses the standard LSP workspace configuration protocol

### How It Works

1. **Workspace Configuration**: Each workspace can have its own disabled services configuration in the `analysis.disabledLanguageServices` field of the LSP analysis config.

2. **Request Handling**: When a language service request is received (e.g., hover, completion), the server checks the workspace configuration for the file being edited.

3. **Service Availability**: If a service is disabled for that workspace, the server returns `None` and logs that the request was skipped.

4. **Configuration Changes**: When workspace configuration changes, the new settings take effect immediately for subsequent requests.

## Use Cases

### Scenario 1: Performance Optimization

If you're working on a very large codebase and find that certain language services are slow, you can disable them to improve performance:

```json
{
  "pyrefly.analysis.disabledLanguageServices": {
    "references": true,
    "workspaceSymbol": true
  }
}
```

### Scenario 2: Conflict Resolution

If you're using multiple language servers and want to use Pyrefly for type checking but another server for certain features:

```json
{
  "pyrefly.analysis.disabledLanguageServices": {
    "completion": true,
    "hover": true
  }
}
```

### Scenario 3: Minimal Mode

For a minimal setup with only type checking and go-to-definition:

```json
{
  "pyrefly.analysis.disabledLanguageServices": {
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

### Scenario 4: Per-Workspace Configuration

You can have different configurations for different workspace folders:

```json
{
  "folders": [
    {
      "path": "/path/to/project1",
      "settings": {
        "pyrefly.analysis.disabledLanguageServices": {
          "hover": true
        }
      }
    },
    {
      "path": "/path/to/project2",
      "settings": {
        "pyrefly.analysis.disabledLanguageServices": {
          "completion": true
        }
      }
    }
  ]
}
```

## Testing

The implementation includes tests to ensure:
- Workspace configurations are correctly parsed
- Disabled services return early from request handlers
- The LSP server functions correctly with services disabled
- Configuration changes take effect dynamically

Run tests with:
```bash
cargo test --package pyrefly --lib
```

