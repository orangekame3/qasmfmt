# qasmfmt

**qasmfmt** is a formatter for OpenQASM 3.0 quantum circuit files.

## Features

- Automatic formatting of OpenQASM 3.0 code
- Comment preservation
- CLI and library interfaces

## Quick Start

```bash
# Install
cargo install qasmfmt

# Format a file (in-place)
qasmfmt input.qasm

# Check formatting
qasmfmt --check input.qasm
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

## License

MIT License
