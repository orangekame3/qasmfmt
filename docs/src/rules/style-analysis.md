# OpenQASM 3.0 Style Analysis

> **Analysis Date:** January 2026

This document summarizes the analysis of examples from the official [openqasm/openqasm](https://github.com/openqasm/openqasm) repository.

## Analyzed Files

- teleport.qasm
- qft.qasm
- rb.qasm
- gateteleport.qasm
- rus.qasm
- inverseqft1.qasm

---

## Observed Style Patterns

### 1. Comments

```qasm
// Line comment
/* Block comment */
```

### 2. OPENQASM Declaration

```qasm
OPENQASM 3;  // With semicolon, without dot
```

Sometimes omitted (some examples start with include)

### 3. Include Statement

```qasm
include "stdgates.inc";
```

- Space between `include` and filename

### 4. Declarations

```qasm
qubit[4] q;
bit[4] c;
const int[32] n = 3;
```

- No space between type and `[`
- Space between `]` and identifier

### 5. Gate Calls

```qasm
h q[0];
cx q[0], q[1];
cphase(pi / 2) q[1], q[0];
rz(pi/4) a;
```

- Space between gate name and arguments
- Space after comma
- **Inconsistent** spacing inside parameters (`pi / 2` and `pi/4` both appear)

### 6. Gate/Def Definitions

```qasm
gate post q { }

def segment(qubit[2] anc, qubit psi) -> bit[2] {
  bit[2] b;
  reset anc;
  ...
  return b;
}
```

- Space before braces
- Body indented by 2 spaces

### 7. If Statement

```qasm
if(c0==1) z q[2];
if(c1==1) { x q[2]; }
if (r == 1) z q;
```

**Inconsistent points:**
- `if(` vs `if (` - presence/absence of space
- Spacing around `==`
- Braces for single statements

### 8. While Statement

```qasm
while(int[2](flags) != 0) {
  flags = segment(ancilla, input_qubit);
}
```

### 9. Measure

```qasm
c0 = measure q[0];     // Assignment form
measure q -> c;        // Arrow form
```

Both forms are used

### 10. Barrier

```qasm
barrier q;
```

---

## Inconsistent Points (to be unified by qasmfmt)

| Item                         | Official Examples | qasmfmt Policy               |
| ---------------------------- | ----------------- | ---------------------------- |
| `if(` vs `if (`              | Mixed             | Unify to `if (`              |
| Spacing around `==`          | Mixed             | Spaces on both sides of `==` |
| Spacing inside parameters    | Mixed             | Unify to `pi / 2`            |
| Braces for single statements | Mixed             | Allow without braces         |

---

## Proposed qasmfmt Rules

### Confirmed Rules (consistent in official examples)

```
✓ Space after include
✓ Space between type and identifier (qubit[2] q)
✓ Space between gate name and arguments (h q)
✓ Space after comma
✓ 2-space indent inside blocks
✓ Space before braces
```

### Normalization Rules (qasmfmt will enforce)

```
→ Space after if ( and while (
→ Spaces around comparison operators (== != < >)
→ Spaces around arithmetic operators (+ - * /)
→ Spaces around assignment operator (=)
```

### Configurable (future)

```
? Indentation: 2 or 4 spaces
? Maximum line width: 80 or 100
```
