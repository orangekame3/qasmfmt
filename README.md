# qasmfmt

**qasmfmt** is a fast formatter for OpenQASM 3.0 quantum circuit files, written in Rust.

## Features

- Fast formatting powered by Rust
- Available as both Rust and Python package
- CLI and library interfaces
- stdin/stdout support for pipeline usage
- Configuration file support (`qasmfmt.toml`)
- Directory recursive processing

## Installation

### Using pip (recommended)

```bash
pip install qasmfmt
```

### Using Cargo

```bash
cargo install qasmfmt
```

### From source

```bash
git clone https://github.com/orangekame3/qasmfmt
cd qasmfmt
cargo install --path .
```

## Usage

### CLI

```bash
qasmfmt [OPTIONS] [PATH]...
```

#### Modes (mutually exclusive)

| Option        | Description                                                     |
| ------------- | --------------------------------------------------------------- |
| `-w, --write` | Write formatted output back to files (in-place)                 |
| `--check`     | Check if files are formatted (exit 1 if not)                    |
| `--diff`      | Show unified diff of formatting changes (exit 1 if diff exists) |

#### Options

| Option                    | Description                                    |
| ------------------------- | ---------------------------------------------- |
| `-i, --indent <N>`        | Indentation size in spaces (default: 4)        |
| `--max-width <N>`         | Maximum line width (default: 100)              |
| `--stdin-filename <PATH>` | Virtual filename for stdin input               |
| `--config <PATH>`         | Path to configuration file                     |
| `--no-config`             | Disable automatic configuration file discovery |
| `-V, --version`           | Print version                                  |
| `-h, --help`              | Print help                                     |

#### Examples

```bash
# Format file (print to stdout)
qasmfmt input.qasm

# Format file in-place
qasmfmt -w input.qasm

# Check if file is formatted (for CI)
qasmfmt --check input.qasm

# Show diff
qasmfmt --diff input.qasm

# Format from stdin
echo 'OPENQASM 3.0;qubit[2]q;' | qasmfmt
echo 'OPENQASM 3.0;qubit[2]q;' | qasmfmt -

# Format from stdin with virtual filename
echo 'OPENQASM 3.0;qubit[2]q;' | qasmfmt --stdin-filename circuit.qasm -

# Custom indent size
qasmfmt -i 2 input.qasm

# Format all .qasm files in a directory (recursive)
qasmfmt -w ./circuits/

# Check all .qasm files in a directory
qasmfmt --check ./src/

# Use specific config file
qasmfmt --config ./qasmfmt.toml input.qasm

# Disable config file auto-discovery
qasmfmt --no-config input.qasm
```

### Configuration File

qasmfmt automatically searches for `qasmfmt.toml` from the input file's directory upward.

```toml
# qasmfmt.toml
indent_size = 2
max_width = 80
indent_style = "spaces"  # or "tabs"
trailing_newline = true
```

CLI options override configuration file settings.

### Python Library

```python
import qasmfmt

# Format string
source = "OPENQASM 3.0;qubit[2]q;h q[0];"
formatted = qasmfmt.format_str(source)
print(formatted)
# OPENQASM 3.0;
# qubit[2] q;
# h q[0];

# Format with options
formatted = qasmfmt.format_str(source, indent_size=2, max_width=80)

# Format file
formatted = qasmfmt.format_file("circuit.qasm")

# Check if file is formatted
is_formatted = qasmfmt.check_file("circuit.qasm")
```

### Rust Library

```rust
use qasmfmt::{format, format_with_config, FormatConfig};

fn main() {
    let source = "OPENQASM 3.0;qubit[2]q;";

    // Format with default config
    let formatted = format(source).unwrap();

    // Format with custom config
    let config = FormatConfig {
        indent_size: 2,
        ..Default::default()
    };
    let formatted = format_with_config(source, config).unwrap();
}
```

## Example

Before:

```qasm
OPENQASM 3.0;include"stdgates.inc";qubit[2]q;bit[2]c;h q[0];cx q[0],q[1];c=measure q;
```

After:

```qasm
OPENQASM 3.0;
include "stdgates.inc";
qubit[2] q;
bit[2] c;
h q[0];
cx q[0], q[1];
c = measure q;
```

## Exit Codes

| Code | Description                                                          |
| ---- | -------------------------------------------------------------------- |
| 0    | Success                                                              |
| 1    | Error or formatting differences found (`--check`, `--diff`)          |
| 2    | Usage error (e.g., mutually exclusive options, stdin with `--write`) |

## CI Integration

### GitHub Actions

```yaml
- name: Check OpenQASM formatting
  run: |
    pip install qasmfmt
    qasmfmt --check .
```

### pre-commit

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: qasmfmt
        name: qasmfmt
        entry: qasmfmt --check
        language: python
        additional_dependencies: [qasmfmt]
        files: \.qasm$
```

## License

MIT License
