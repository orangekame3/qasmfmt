# Getting Started

## Basic Usage

Format a file and print to stdout:

```bash
qasmfmt input.qasm
```

Format and overwrite the file:

```bash
qasmfmt -w input.qasm
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

## Multiple Files

```bash
qasmfmt -w *.qasm
```

## Show Diff

```bash
qasmfmt --diff input.qasm
```
