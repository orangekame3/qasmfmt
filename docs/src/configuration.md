# Configuration

## Configuration File

qasmfmt looks for `qasmfmt.toml` in the current directory.

```toml
# qasmfmt.toml

# Indent style: "spaces" or "tabs"
indent_style = "spaces"

# Number of spaces per indentation level
indent_size = 4

# Maximum line width
max_width = 100

# Add trailing newline
trailing_newline = true
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `indent_style` | string | `"spaces"` | `"spaces"` or `"tabs"` |
| `indent_size` | integer | `4` | Spaces per indent level |
| `max_width` | integer | `100` | Maximum line width |
| `trailing_newline` | boolean | `true` | Add newline at end of file |

## CLI Override

CLI options override config file settings:

```bash
qasmfmt --indent 2 input.qasm
```
