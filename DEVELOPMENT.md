# qasmfmt Development Documentation

## Overview

`qasmfmt` is a formatter for OpenQASM 3.0 quantum circuit files written in Go. This tool automatically formats QASM code according to standardized rules to improve readability and consistency.

## Architecture

### Core Components

#### 1. Parser Integration
- **Library**: `github.com/itsubaki/qasm`
- **Parser**: ANTLR4-generated OpenQASM 3.0 parser
- **Location**: `cmd/formatter.go`

The formatter uses the ANTLR-generated parser from the `itsubaki/qasm` library to create an Abstract Syntax Tree (AST) of the input QASM code.

#### 2. Formatter Implementation
- **Main Type**: `Formatter` struct
- **Key Method**: `Format(content string) (string, error)`
- **Approach**: AST traversal with rule-based formatting

### API Structure

```go
// Main formatter interface
func FormatQASM(content string) (string, error)
func ValidateQASM(content string) error

// Formatter type
type Formatter struct {
    indentSize int  // Default: 2 spaces
}
```

### Parser Integration Details

The formatter integrates with the ANTLR parser as follows:

```go
// Create lexer and parser
lexer := parser.Newqasm3Lexer(antlr.NewInputStream(content))
p := parser.Newqasm3Parser(antlr.NewCommonTokenStream(lexer, antlr.TokenDefaultChannel))

// Parse into AST
tree := p.Program()

// Format the tree
return f.formatProgram(tree.(*parser.ProgramContext))
```

## Formatting Rules Implementation

Based on `spec.yaml`, the formatter implements these rules:

### 1. Version Declaration
- `OPENQASM 3.0;` placed at the top
- Include statements follow immediately

### 2. Indentation
- 2 spaces per indent level
- Consistent indentation for blocks

### 3. Spacing
- Spaces around binary operators (`=`, `+`, `-`, `*`, `/`, `^`)
- No spaces inside brackets `[]`, parentheses `()`, or braces `{}`
- No space between function name and parentheses

### 4. Statements
- All statements terminated with semicolon
- One statement per line
- No multiple statements on same line

### 5. Blocks
- Opening brace `{` on same line
- Closing brace `}` on new line with matching indentation
- Body content indented

### 6. File Structure
- File ends with newline character
- Empty lines between major blocks

## AST Node Handling

The formatter handles different AST node types:

### Supported Contexts
- `ProgramContext` - Root program node
- `IStatementContext` - Individual statements
- `IGateCallStatementContext` - Gate applications
- `IQuantumDeclarationStatementContext` - Qubit declarations
- `IGateStatementContext` - Gate definitions
- `IAssignmentStatementContext` - Variable assignments
- `IMeasureArrowAssignmentStatementContext` - Measurement statements
- `IIfStatementContext` - Conditional statements

### Example AST Traversal

```go
func (f *Formatter) formatStatement(stmt parser.IStatementContext, indent int) string {
    if gateCall := stmt.GateCallStatement(); gateCall != nil {
        return f.formatGateCall(gateCall, indent)
    }
    
    if quantumDecl := stmt.QuantumDeclarationStatement(); quantumDecl != nil {
        return f.formatQuantumDeclaration(quantumDecl, indent)
    }
    
    // Handle other statement types...
}
```

## CLI Interface

### Commands
- `qasmfmt format [file...]` - Format QASM files
- `qasmfmt check [file...]` - Check QASM syntax

### Flags
- `-w, --write` - Write result to source file
- `-d, --diff` - Display differences instead of rewriting

### Usage Examples

```bash
# Format and output to stdout
qasmfmt format example.qasm

# Format in-place
qasmfmt format -w example.qasm

# Show differences
qasmfmt format -d example.qasm

# Check syntax only
qasmfmt check example.qasm
```

## Dependencies

### Runtime Dependencies
- `github.com/itsubaki/qasm@v0.1.1` - OpenQASM 3.0 parser with ANTLR4 support
- `github.com/antlr4-go/antlr/v4@v4.13.1` - ANTLR runtime for Go
- `github.com/spf13/cobra@v1.9.1` - CLI framework
- `github.com/charmbracelet/fang@v0.2.0` - Enhanced CLI experience

### Development Dependencies
- `github.com/golangci/golangci-lint` - Comprehensive Go linter
- `mvdan.cc/gofumpt` - Stricter Go formatter
- Task runner for build automation

### System Requirements
- **Go Version**: 1.24.4 or later
- **Platforms**: Linux (amd64), macOS (amd64/arm64), Windows (amd64)
- **Memory**: Minimum 512MB for large QASM files

## Testing Strategy

### Test Cases Needed
1. **Basic Formatting**
   - Simple gate applications
   - Qubit declarations
   - Measurement statements

2. **Complex Structures**
   - Gate definitions with parameters
   - Nested control flow
   - Multiple include statements

3. **Edge Cases**
   - Empty files
   - Comments preservation
   - Malformed syntax

4. **Compliance Testing**
   - Validate against official OpenQASM examples
   - Cross-check with spec.yaml rules

### Example Test Structure

```go
func TestFormatBasicGates(t *testing.T) {
    input := `OPENQASM 3.0;qubit[2] q;h q[0];cx q[0],q[1];`
    expected := `OPENQASM 3.0;
qubit[2] q;
h q[0];
cx q[0], q[1];
`
    result, err := FormatQASM(input)
    assert.NoError(t, err)
    assert.Equal(t, expected, result)
}
```

## Known Limitations

### Current Limitations
1. **Comment Preservation**: Comments are not yet preserved during formatting
2. **Complex Expressions**: Some complex mathematical expressions may not format optimally
3. **Error Recovery**: Limited error recovery for malformed QASM files

### Future Improvements
1. **Comment Handling**: Implement comment preservation using ANTLR comment channels
2. **Expression Formatting**: Enhanced formatting for mathematical expressions
3. **Error Messages**: Better error reporting with line numbers and context
4. **Configuration**: Support for custom formatting rules via config file

## Development Workflow

### Adding New Features
1. Update `spec.yaml` if adding new formatting rules
2. Implement formatting logic in `cmd/formatter.go`
3. Add corresponding test cases
4. Update documentation

### Code Style Guidelines
- Follow Go conventions
- Use meaningful variable names
- Add comments for complex formatting logic
- Maintain consistent error handling

### Development Environment Setup

#### Prerequisites
- Go 1.24.4 or later
- [Task](https://taskfile.dev/) for task automation (recommended)

#### Initial Setup
```bash
# Install Task (if not already installed)
# macOS
brew install go-task/tap/go-task

# Linux/WSL
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d

# Setup development environment
task setup
```

### Task-based Development Workflow

This project uses [Task](https://taskfile.dev/) for automation. All major development tasks are defined in `Taskfile.yaml`.

#### Core Development Tasks

```bash
# Show all available tasks
task

# Build the project
task build

# Build for all platforms
task build:all

# Run in development mode
task dev

# Install to $GOPATH/bin
task install
```

#### Testing and Quality

```bash
# Run tests
task test

# Run tests with coverage
task test:coverage

# Run tests with race detection
task test:race

# Run benchmarks
task bench

# Format code
task fmt

# Run linters
task lint

# Run complete CI pipeline locally
task ci
```

#### Dependency Management

```bash
# Download dependencies
task deps

# Tidy dependencies
task deps:tidy

# Verify dependencies
task deps:verify

# Update all dependencies
task deps:update
```

#### Demo and Examples

```bash
# Run formatter demo
task demo

# Show formatting differences
task demo:diff

# Create example QASM files
task example:create
```

#### Build and Release

```bash
# Build for specific platforms
task build:linux
task build:darwin
task build:windows

# Create release builds
task release

# Show version information
task version
```

#### Development Environment Management

```bash
# Setup development environment
task setup

# Clean build artifacts
task clean

# Clean all generated files including caches
task clean:all
```

### Manual Build and Test Commands

If you prefer not to use Task, you can run commands manually:

```bash
# Build the project
go build

# Run tests (when implemented)
go test ./...

# Format code
go fmt ./...

# Check for issues
go vet ./...

# Install linters
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Run linters
golangci-lint run
```

## Integration with Editor/IDE

### Future Integration Plans
- **VS Code Extension**: Language server integration
- **Vim Plugin**: Format on save functionality
- **GitHub Actions**: Automated formatting checks in CI/CD

### Language Server Protocol
Consider implementing LSP support for:
- Real-time formatting
- Syntax error highlighting
- Auto-completion for QASM keywords

## Contributing

### Guidelines for Contributors
1. Follow the established architecture patterns
2. Add tests for new functionality
3. Update documentation for significant changes
4. Ensure backwards compatibility when possible

### Code Review Checklist
- [ ] Follows formatting rules from spec.yaml
- [ ] Includes appropriate test coverage
- [ ] Handles edge cases gracefully
- [ ] Maintains performance for large files
- [ ] Updates documentation as needed

## Taskfile.yaml Reference

The project uses Task for build automation. Here's a comprehensive list of available tasks:

### Build Tasks
- `task build` - Build the application with version info
- `task build:all` - Build for all supported platforms
- `task build:linux` - Build for Linux (amd64)
- `task build:darwin` - Build for macOS (amd64 and arm64)
- `task build:windows` - Build for Windows (amd64)
- `task install` - Install binary to $GOPATH/bin

### Development Tasks
- `task dev` - Run in development mode with live reload
- `task demo` - Run formatter on example file
- `task demo:diff` - Show formatting differences
- `task demo:write` - Format example file in place
- `task example:create` - Create additional example QASM files

### Testing and Quality Tasks
- `task test` - Run unit tests
- `task test:coverage` - Run tests with coverage report
- `task test:race` - Run tests with race detection
- `task bench` - Run benchmarks
- `task lint` - Run all linters (fmt, vet, golangci-lint)
- `task lint:golangci` - Run golangci-lint only
- `task lint:install` - Install golangci-lint
- `task fmt` - Format code with gofumpt
- `task fmt:install` - Install gofumpt

### Dependency Management Tasks
- `task deps` - Download dependencies
- `task deps:tidy` - Tidy dependencies
- `task deps:verify` - Verify dependencies
- `task deps:update` - Update all dependencies

### CI/CD Tasks
- `task ci` - Run complete CI pipeline locally
- `task release` - Create release builds
- `task pre-commit` - Run pre-commit checks

### Utility Tasks
- `task clean` - Clean build artifacts
- `task clean:all` - Clean all generated files including caches
- `task setup` - Setup development environment
- `task version` - Show version information

### Variables and Configuration

The Taskfile uses these variables:
- `BINARY_NAME`: qasmfmt
- `BUILD_DIR`: bin
- `VERSION`: Auto-generated from git tags
- `COMMIT`: Auto-generated from git commit hash
- `BUILD_DATE`: Auto-generated timestamp

Build flags are automatically injected:
```
-ldflags "-X main.version={{.VERSION}} -X main.commit={{.COMMIT}} -X main.buildDate={{.BUILD_DATE}}"
```

### Configuration Files

#### `.golangci.yml`
Linter configuration with enabled rules:
- Standard Go linters (errcheck, gosimple, govet, etc.)
- Code quality checks (gocyclo, gocritic)
- Security checks (gosec)
- Style consistency (revive, whitespace)

Settings:
- Minimum complexity threshold: 15
- Confidence level: low for security checks
- Test files have relaxed rules

## Project Status and Roadmap

### Completed ✅
- [x] Basic OpenQASM 3.0 parser integration
- [x] CLI interface with format and check commands
- [x] Build system with cross-platform support
- [x] Development workflow automation
- [x] Code quality tools setup
- [x] Go 1.24.4 compatibility

### In Progress 🚧
- [ ] Formatter spacing and layout improvements
- [ ] Comment preservation during formatting

### Planned 📋
- [ ] Comprehensive test suite
- [ ] Error message improvements
- [ ] Configuration file support
- [ ] Editor/IDE integrations
- [ ] Performance optimizations
- [ ] Additional OpenQASM 3.0 features

---

This documentation serves as a foundation for ongoing development and maintenance of the qasmfmt tool.