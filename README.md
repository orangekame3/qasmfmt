# qasmfmt

**qasmfmt** is a fast formatter for OpenQASM 3.0 quantum circuit files, written in Rust.

## Features

- Fast formatting powered by Rust
- Available as both Rust and Python package
- CLI and library interfaces
- stdin/stdout support for pipeline usage

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
# Format file (print to stdout)
qasmfmt input.qasm

# Format file in-place
qasmfmt -w input.qasm
qasmfmt --write input.qasm

# Check if file is formatted (for CI)
qasmfmt -c input.qasm
qasmfmt --check input.qasm

# Show diff
qasmfmt -d input.qasm
qasmfmt --diff input.qasm

# Format from stdin
echo 'OPENQASM 3.0;qubit[2]q;' | qasmfmt

# Custom indent size
qasmfmt -i 2 input.qasm
qasmfmt --indent 2 input.qasm

# Format multiple files
qasmfmt -w *.qasm
```

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

## CI Integration

### GitHub Actions

```yaml
- name: Check OpenQASM formatting
  run: |
    pip install qasmfmt
    qasmfmt --check **/*.qasm
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
