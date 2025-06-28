# Parser Issues and Limitations

## Library Information

- **Parser Type**: ANTLR4-generated OpenQASM 3.0 parser
- **Go Module**: Uses `github.com/antlr4-go/antlr/v4@v4.13.1`

## Identified Issues

### 1. Comment Preservation

**Issue**: Comments are not accessible through the standard ANTLR token stream
**Status**: ❌ Not resolved

**Details**:

- Comments (`//` and `/* */`) are completely stripped during parsing
- ANTLR typically sends comments to a hidden channel, but they're not accessible via `tokenStream.GetAllTokens()`
- Attempted to extract comments using:

```go
for _, token := range tokens {
    if token.GetChannel() != antlr.TokenDefaultChannel {
        // No comment tokens found
    }
}
```

**Impact**:

- Comments in source files are lost during formatting
- Users must manually re-add comments after formatting

**Workaround Implemented**:

- Basic comment structure detection framework added
- Placeholder for future comment preservation features

**Potential Solutions**:

1. Fork the parser and modify lexer rules to preserve comments
2. Use a different OpenQASM parser with better comment support
3. Implement pre/post-processing to extract and re-inject comments

### 2. Statement Type Recognition Gaps

**Issue**: Some statement types not properly categorized by the parser
**Status**: ✅ Partially resolved with workarounds

**Details**:

- Classical bit declarations (`bit c;`) initially not recognized as `ClassicalDeclarationStatement`
- Required detailed AST exploration to identify correct context types:

```go
// Originally missing this check:
if classicalDecl := stmt.ClassicalDeclarationStatement(); classicalDecl != nil {
    return f.formatClassicalDeclaration(classicalDecl, indent)
}
  ```

**Resolution**:

- Added comprehensive statement type checking
- Implemented proper handling for `IClassicalDeclarationStatementContext`

### 3. Malformed Code Parsing Errors

**Issue**: Parser fails completely on common malformed patterns
**Status**: ✅ Resolved with preprocessing

**Examples of Problematic Input**:

```qasm
// These cause parser errors:
include"stdgates.qasm";          // Missing space
qubit[2]q;                       // Missing space
hq[0];                           // Missing space
cxq[0],q[1];                     // Missing spaces
measureq->c;                     // Missing spaces
```

**Error Messages**:

```
line 1:60 no viable alternative at input 'cxq[0],'
line 1:59 mismatched input ']' expecting ';'
line 4:8 no viable alternative at input 'measureq->'
```

**Resolution**:

- Implemented comprehensive preprocessing in `preprocessMalformedQASM()`
- Added pattern-based fixes for common malformed structures
- Split compound statements into separate lines

### 4. Token Channel Configuration

**Issue**: Hidden channel tokens (comments, whitespace) not preserved
**Status**: ❌ Not resolved

**Technical Details**:

- ANTLR lexer configuration doesn't expose comment tokens
- `antlr.TokenDefaultChannel` only includes syntax tokens
- No access to `HIDDEN` channel tokens containing comments and formatting

**Debug Output**:

```
Total tokens: 7
Token[0]: Type=1, Channel=0, Text="OPENQASM", Line=1
Token[1]: Type=103, Channel=0, Text="3.0", Line=1
Token[2]: Type=63, Channel=0, Text=";", Line=1
// No comment tokens present
```

### 5. Complex Expression Handling

**Issue**: Limited support for complex mathematical expressions
**Status**: 🚧 Partially addressed

**Examples**:

- Complex arithmetic expressions in gate parameters
- Nested function calls
- Mathematical constants and functions

**Current Limitation**:

- Basic expression formatting works
- Complex expressions may not format optimally
- Fall back to generic text processing

### 6. Gate Definition Complexity

**Issue**: Complex gate definitions with parameters not fully supported
**Status**: 🚧 Basic support implemented

**Examples**:

```qasm
gate custom_gate(theta, phi) q1, q2 {
    rz(theta) q1;
    cx q1, q2;
    rz(phi) q2;
}
```

**Current Limitation**:

- Basic gate definition structure recognized
- Parameter formatting may not be optimal
- Complex gate body statements need improvement

## API Documentation Gaps

### Missing Context Types

During development, we discovered these context types through experimentation:

**Working Context Types**:

- `IQuantumDeclarationStatementContext`
- `IClassicalDeclarationStatementContext`
- `IGateCallStatementContext`
- `IMeasureArrowAssignmentStatementContext`
- `IIfStatementContext`
- `IGateStatementContext`

**Uncertain/Undocumented**:

- `IForStatementContext` - existence unconfirmed
- `IWhileStatementContext` - existence unconfirmed
- `IFunctionDeclarationContext` - existence unconfirmed

## Performance Considerations

### Parser Performance

- **Parsing Speed**: Generally fast for typical QASM files
- **Memory Usage**: Reasonable for files under 10MB
- **Error Recovery**: Limited - single error can fail entire parse

### Preprocessing Overhead

- **Regex Processing**: Multiple regex passes add ~5-10% overhead
- **String Manipulation**: Acceptable for typical use cases
- **Memory Allocation**: Minimal impact for standard files

## Recommendations for Future Development

### Short Term (Next Release)

1. **Improve Error Messages**: Add line number and context information
2. **Expand Preprocessing**: Handle more edge cases in malformed code
3. **Better Expression Handling**: Improve mathematical expression formatting

### Medium Term

1. **Alternative Parser Evaluation**: Research other OpenQASM parsers with better comment support
2. **Custom Lexer Rules**: Consider forking to add comment preservation
3. **Configuration System**: Allow users to control formatting rules

### Long Term

1. **Language Server Protocol**: Implement LSP for real-time formatting
2. **AST-based Transformations**: More sophisticated code transformations
3. **Plugin Architecture**: Allow custom formatting rules

## Workarounds Implemented

### 1. Preprocessing Pipeline

```go
func (f *Formatter) preprocessMalformedQASM(content string) string {
    content = f.splitCompoundStatements(content)
    // Apply pattern-based fixes
    // Return cleaned content for parsing
}
```

### 2. Fallback Formatting

```go
// When specific context handling fails:
text := strings.TrimSpace(stmt.GetText())
return f.indent(indent) + f.addSpacesAroundOperators(text) + ";"
```

### 3. Comprehensive Statement Detection

```go
// Check all possible statement types:
if quantumDecl := stmt.QuantumDeclarationStatement(); quantumDecl != nil {
    return f.formatQuantumDeclaration(quantumDecl, indent)
}
if classicalDecl := stmt.ClassicalDeclarationStatement(); classicalDecl != nil {
    return f.formatClassicalDeclaration(classicalDecl, indent)
}
// ... continue for all types
```

## Testing Strategy for Parser Issues

### Unit Tests for Edge Cases

- Malformed input patterns
- Comment preservation (when implemented)
- Complex expression handling
- Error recovery scenarios

### Integration Tests

- Real-world QASM file formatting
- Performance benchmarks
- Cross-platform compatibility

### Regression Tests

- Ensure fixes don't break existing functionality
- Maintain backwards compatibility
- Test with various OpenQASM file sizes

## Contributing Notes

When contributing to this project, please:

1. **Test with Real Files**: Use actual OpenQASM files from quantum projects
2. **Document New Issues**: Add any newly discovered parser limitations here
3. **Maintain Workarounds**: Ensure fallback mechanisms continue working
4. **Performance Testing**: Verify changes don't significantly impact parsing speed

---

## Last Updated: 2025-06-28
