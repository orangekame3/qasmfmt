# qasmfmt

A command-line formatter for OpenQASM 3.0 quantum circuit files written in Go.

![qasmfmt demo](./img/demo.gif)

## Features

- **Format OpenQASM 3.0 files** with proper indentation and spacing
- **Syntax validation** for quantum circuit code
- **Diff output** to show formatting changes
- **Cross-platform support** (Linux, macOS, Windows)
- **Fast parsing** using ANTLR4-generated parser

## Installation

### Using Homebrew (for macOS and Linux)

```bash
brew install orangekame3/tap/qasmfmt
```

### From Source

```bash
# Clone the repository
git clone https://github.com/orangekame3/qasmfmt.git
cd qasmfmt

# Build the binary
task build

# Or install to $GOPATH/bin
task install
```

### Using Go

```bash
go install github.com/orangekame3/qasmfmt@latest
```

## Usage

### Basic Usage

```bash
# Format and output to stdout
qasmfmt example.qasm

# Format in-place
qasmfmt -w example.qasm

# Check if file is formatted
qasmfmt -c example.qasm

# Show formatting differences
qasmfmt -d example.qasm
```

### Options

```bash
# Custom indentation (default: 2 spaces)
qasmfmt -i 4 example.qasm

# Disable trailing newline
qasmfmt --newline=false example.qasm

# Verbose output
qasmfmt -v -w example.qasm

# Format multiple files
# Similar to `go fmt`, `qasmfmt` can process multiple files or directories.
qasmfmt -w *.qasm # or qasmfmt -w . or qasmfmt -w ./... to format all .qasm files in the current directory and its subdirectories.
```

### Subcommands (Alternative Syntax)

```bash
# Format command (same as direct usage)
qasmfmt format -w example.qasm

# Check command
qasmfmt check example.qasm

# Generate shell completions
qasmfmt completion bash > /usr/local/etc/bash_completion.d/qasmfmt

# Show version
qasmfmt version
```

## Example

**Before formatting:**

```qasm
OPENQASM 3.0;include"stdgates.qasm";qubit[2]q;hq[0];cxq[0],q[1];
```

**After formatting:**

```qasm
OPENQASM 3.0;
include "stdgates.qasm";
qubit[2] q;
h q[0];
cx q[0], q[1];
```

## Formatting Rules

qasmfmt follows the OpenQASM 3.0 formatting specification:

- **Version Declaration**: `OPENQASM 3.0;` at the top
- **Include Statements**: Proper spacing around include directives
- **Indentation**: Configurable spaces per level (default: 2)
- **Spacing**: Spaces around operators and after commas
- **Statements**: One statement per line with semicolon termination
- **File Ending**: Configurable trailing newline (default: enabled)

## Development

### Prerequisites

- Go 1.24.4 or later
- [Task](https://taskfile.dev/) (recommended for development)

### Quick Start

```bash
# Setup development environment
task setup

# Run tests
task test

# Build and test
task build
task demo

# Run linters
task lint

# See all available tasks
task
```

### Project Structure

```bash
qasmfmt/
├── cmd/                    # CLI commands and formatter logic
│   ├── format.go          # Format command implementation
│   ├── check.go           # Check command implementation
│   ├── formatter.go       # Core formatting logic
│   └── formatter_test.go  # Tests
├── examples/              # Example QASM files
├── Taskfile.yaml         # Task automation
├── DEVELOPMENT.md        # Development documentation
└── spec.yaml            # Formatting rules specification
```

## Supported OpenQASM 3.0 Features

- ✅ Version declarations (`OPENQASM 3.0;`)
- ✅ Include statements (`include "stdgates.qasm";`)
- ✅ Qubit declarations (single and arrays: `qubit q;`, `qubit[2] q;`)
- ✅ Classical bit declarations (`bit c;`, `bit[2] c;`)
- ✅ Gate calls (single and multi-qubit: `h q[0];`, `cx q[0], q[1];`)
- ✅ Measurement statements (`measure q -> c;`)
- ✅ Basic expressions and operators
- ✅ Malformed code preprocessing and repair
- ✅ Intelligent whitespace and line spacing
- ✅ Cross-platform binary builds
- 🚧 Comment preservation (limited support)
- 🚧 Complex gate definitions (basic support)
- 🚧 Control flow statements (basic support)

## Contributing

Contributions are welcome! Please see [DEVELOPMENT.md](DEVELOPMENT.md) for development guidelines.

### Development Workflow

1. **Fork and clone** the repository
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make changes** and add tests
4. **Run tests**: `task test`
5. **Run linters**: `task lint`
6. **Commit changes**: `git commit -am 'Add my feature'`
7. **Push to branch**: `git push origin feature/my-feature`
8. **Create Pull Request**

## License

[MIT License](LICENSE)

## Related Projects

- [OpenQASM](https://github.com/openqasm/openqasm) - OpenQASM specification
