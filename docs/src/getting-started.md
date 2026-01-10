# Getting Started

## Basic Usage

Format a file (in-place):

```bash
qasmfmt input.qasm
```

Format multiple files:

```bash
qasmfmt *.qasm
```

Format from stdin:

```bash
cat input.qasm | qasmfmt
```

## Check Mode

Check if files are already formatted (useful for CI):

```bash
qasmfmt --check input.qasm
```

Exit codes:
- `0`: File is formatted
- `1`: File needs formatting
- `2`: Error (parse error, file not found, etc.)

## Show Diff

Preview changes without modifying files:

```bash
qasmfmt --diff input.qasm
```
