# Architecture

## Overview

qasmfmt uses a pipeline architecture inspired by rustfmt:

```
Source Code
    ↓
┌─────────────┐
│   Parser    │  ← oq3_syntax
└─────────────┘
    ↓
┌─────────────┐
│     AST     │
└─────────────┘
    ↓
┌─────────────┐
│  IR (Doc)   │  ← Intermediate Representation
└─────────────┘
    ↓
┌─────────────┐
│   Printer   │  ← Pretty Printer
└─────────────┘
    ↓
Formatted Code
```

## Modules

| Module | Description |
|--------|-------------|
| `config` | Configuration handling |
| `error` | Error types |
| `ir` | Intermediate representation (Doc) |
| `printer` | Pretty printer |
| `format` | AST to IR conversion |
| `comment` | Comment extraction |

## IR (Doc)

The intermediate representation is based on Wadler's "A prettier printer":

```rust
enum Doc {
    Nil,
    Text(String),
    Hardline,
    Softline,
    Concat(Vec<Doc>),
    Indent(Box<Doc>),
    Group(Box<Doc>),
}
```

## Dependencies

- `oq3_syntax` - OpenQASM 3.0 parser
- `serde` - Serialization
- `thiserror` - Error handling
