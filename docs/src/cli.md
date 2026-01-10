# CLI Reference

## Synopsis

```
qasmfmt [OPTIONS] [FILES...]
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--write` | `-w` | Write formatted output back to source file |
| `--check` | `-c` | Check if files are formatted (exit 1 if not) |
| `--diff` | `-d` | Show unified diff |
| `--indent <N>` | `-i` | Number of spaces per indentation level (default: 4) |
| `--max-width <N>` | | Maximum line width (default: 100) |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Examples

```bash
# Format to stdout
qasmfmt input.qasm

# Format in place
qasmfmt -w input.qasm

# Check formatting (for CI)
qasmfmt -c input.qasm

# Custom indentation
qasmfmt --indent 2 input.qasm

# Show diff
qasmfmt -d input.qasm

# Stdin
echo "OPENQASM 3.0;qubit q;" | qasmfmt
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | File needs formatting (with `--check`) |
| 2 | Error (parse error, IO error, etc.) |
