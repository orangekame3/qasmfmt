# qasmfmt TODO - OpenQASM 3.0 Feature Support

Based on analysis of the [OpenQASM 3.0 specification](https://github.com/openqasm/openqasm), this document outlines missing features and improvements needed for comprehensive OpenQASM support.

## Current Status
qasmfmt supports basic OpenQASM 3.0 formatting and has been extended with several additional features. See [README.md](README.md#supported-openqasm-30-features) for current capabilities.

### ✅ Recently Completed Features
- **Barrier statements**: `barrier;`, `barrier q;`, `barrier q[0], q[1];`
- **Reset statements**: `reset q;`, `reset q[0];`
- **Parameterized gates**: `rz(pi/4) q[0];`, `cphase(pi/2) q[0], q[1];`
- **Comment preservation infrastructure**: Framework added (limited by lexer)

## 🚀 High Priority Features

### Advanced Gate Definitions
- [ ] **Custom gate definitions with parameters**
  - Currently: Basic gate definition support (placeholders)
  - Needed: Full gate body formatting, parameter handling
  - Example: `gate rz(theta) q { U(0, 0, theta) q; }`

### Function and Subroutine Support
- [ ] **Function definitions and calls**
  - Currently: Not supported
  - Needed: `def` function formatting, parameter lists, return types
  - Example: `def majority(qubit a, qubit b, qubit c) { /* body */ }`

- [ ] **Subroutine definitions**
  - Currently: Not supported  
  - Needed: Classical subroutines with proper formatting

### Control Flow Statements
- [ ] **If/else statements**
  - Currently: Basic support (placeholder formatting)
  - Needed: Proper conditional formatting, nested blocks

- [ ] **For loops**
  - Currently: Not supported
  - Needed: Loop formatting with proper indentation

- [ ] **While loops**
  - Currently: Not supported
  - Needed: While loop formatting and condition handling

### Advanced Data Types and Arrays
- [ ] **Multi-dimensional arrays**
  - Currently: Not supported
  - Needed: Array declaration formatting: `array[int[8], 16] my_ints;`

- [ ] **Array operations and indexing**
  - Currently: Not supported
  - Needed: Slice notation: `my_array[0:1]`, multi-index access

- [ ] **Complex types (int, uint, float with bit widths)**
  - Currently: Basic bit/qubit only
  - Needed: `int[32]`, `uint[16]`, `float[64]` formatting

## 🔧 Medium Priority Features

### Calibration and Timing
- [ ] **defcal statements**
  - Currently: Not supported
  - Needed: Calibration definition formatting
  - Example: `defcal x $0 { play drive($0), gaussian(...); }`

- [ ] **Timing constructs**
  - Currently: Not supported
  - Needed: `delay`, `durationof`, timing expressions

- [ ] **Pulse definitions**
  - Currently: Not supported
  - Needed: `play`, `capture`, `shift_phase` formatting

### Advanced Gate Operations
- [x] **Parameterized gates**
  - ✅ **COMPLETED**: Gates with expressions: `rz(pi/4)`, `cphase(theta)`
  - Supports single and multi-qubit parameterized gates with proper spacing

- [ ] **Gate modifiers**
  - Currently: Not supported
  - Needed: `ctrl @`, `inv @`, `pow(n) @` formatting

- [x] **Barrier statements**
  - ✅ **COMPLETED**: `barrier;`, `barrier q;`, `barrier q[0], q[1];` formatting

### Classical Computation
- [ ] **Classical expressions and operations**
  - Currently: Basic assignment support
  - Needed: Complex arithmetic, function calls

- [ ] **Constant definitions**
  - Currently: Not supported
  - Needed: `const` declarations

- [ ] **Type casting and conversions**
  - Currently: Not supported
  - Needed: Type conversion formatting

## 🎯 Low Priority Features

### Reset and Initialization
- [x] **Reset statements**  
  - ✅ **COMPLETED**: `reset q;`, `reset q[0];` formatting
  - Supports proper spacing and qubit indexing

### Advanced Language Constructs
- [ ] **extern function declarations**
  - Currently: Not supported
  - Needed: External function interface formatting

- [ ] **pragma statements**  
  - Currently: Not supported
  - Needed: Compiler directive formatting

- [ ] **switch/case statements**
  - Currently: Not supported
  - Needed: Switch statement formatting

### Enhanced Comment Support
- [x] **Comment preservation and positioning**
  - ⚠️ **PARTIALLY COMPLETED**: Infrastructure added, limited by lexer grammar
  - Current limitation: Comments are skipped in lexer, requires grammar modification

- [ ] **Multi-line comment formatting**
  - Currently: Limited support
  - Needed: `/* */` block comment formatting

### Error Handling and Diagnostics
- [ ] **Better error messages**
  - Currently: Basic parse errors
  - Needed: Detailed syntax error reporting with line numbers

- [ ] **Syntax recovery**
  - Currently: Basic malformed code repair
  - Needed: More sophisticated error recovery

## 🔄 Code Quality Improvements

### Parser and Formatter Enhancements
- [ ] **Update ANTLR grammar to latest OpenQASM spec**
  - Current: Basic OpenQASM 3.0 support
  - Needed: Full spec compliance, latest language features

- [ ] **Improved formatting rules**
  - Current: Basic spacing and indentation
  - Needed: More sophisticated layout, alignment options

- [ ] **Configuration system**
  - Current: Basic indent/newline config
  - Needed: Comprehensive formatting preferences

### Testing and Validation
- [ ] **Extended test suite**
  - Current: Basic formatting tests
  - Needed: Tests for all OpenQASM features

- [ ] **Compliance testing against reference implementations**
  - Current: None
  - Needed: Validation against Qiskit, other OpenQASM parsers

- [ ] **Performance optimization**
  - Current: Adequate for small files
  - Needed: Optimization for large quantum circuits

## 📚 Legacy TODO Items

### Formatting Quality & Readability
- [ ] **Introduction of Semantic Formatting Rules:**
  - Implement rules for aligning multi-line statements (e.g., declarations, assignments) to improve vertical readability
  - Develop intelligent line-breaking strategies for long expressions, function calls, and complex statements to enhance code clarity
  - Consider rules for consistent spacing around operators, keywords, and punctuation

### Usability & Configuration  
- [ ] **Expanded Configuration Options:**
  - Allow users to specify OpenQASM version targets for formatting, enabling compatibility with different specification versions
  - Introduce configurable formatting styles (e.g., indentation size, brace style, maximum line length) to align with various coding standards and personal preferences
  - Provide options to enable/disable specific formatting rules

- [ ] **Integration with Build Systems and IDEs:**
  - VS Code extension for qasmfmt
  - Pre-commit hook templates
  - Better integration with quantum development tools

## References
- [OpenQASM 3.0 Specification](https://github.com/openqasm/openqasm)
- [OpenQASM Examples](https://github.com/openqasm/openqasm/tree/main/examples)
