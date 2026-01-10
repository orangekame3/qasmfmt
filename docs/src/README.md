# qasmfmt

**qasmfmt** is a formatter for OpenQASM 3.0 quantum circuit files.

## Features

- Automatic formatting of OpenQASM 3.0 code
- Configurable indentation and line width
- Comment preservation
- CLI and library interfaces

## Quick Start

```bash
# Install
cargo install qasmfmt

# Format a file
qasmfmt input.qasm

# Format in place
qasmfmt -w input.qasm

# Check formatting
qasmfmt --check input.qasm
```

## Example

Before:

```qasm
OPENQASM 3.0;include"stdgates.qasm";qubit[2]q;bit[2]c;hq[0];cxq[0],q[1];c=measure q;
```

After:

```qasm
OPENQASM 3.0;
include "stdgates.qasm";

qubit[2] q;
bit[2] c;
h q[0];
cx q[0], q[1];
c = measure q;
```

## Status

This project is in early development (v0.0.1). The core formatter is being implemented.

## License

MIT License
