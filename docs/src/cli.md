# CLI Reference

## Synopsis

```
qasmfmt [OPTIONS] [FILES...]
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--check` | | Check if files are formatted (exit 1 if not) |
| `--diff` | | Show diff instead of writing |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Behavior

- **Default**: Format files in-place
- **No files given**: Read from stdin, write to stdout
- **With `--check`**: Exit with 1 if files would be reformatted
- **With `--diff`**: Show diff, do not modify files

## Examples

```bash
# Format files in place
qasmfmt input.qasm
qasmfmt src/*.qasm

# Check if formatted (for CI)
qasmfmt --check input.qasm

# Show diff
qasmfmt --diff input.qasm

# Format from stdin
cat input.qasm | qasmfmt
echo "OPENQASM 3.0;qubit q;" | qasmfmt

# Format all QASM files recursively
find . -name "*.qasm" | xargs qasmfmt
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Would reformat (with `--check`) |
| 2 | Error (parse error, IO error) |
