# Formatting Rules

This document defines the formatting rules for qasmfmt.
Rules are based on analysis of examples from the [official OpenQASM repository](https://github.com/openqasm/openqasm).

---

## 1. Version Declaration

```qasm
// Before
OPENQASM  3.0;

// After
OPENQASM 3.0;
```

- Single space between `OPENQASM` and version number

---

## 2. Include Statement

```qasm
// Before
include"stdgates.inc";

// After
include "stdgates.inc";
```

- Single space between `include` and file path

---

## 3. Declarations

### 3.1 Qubit Declaration

```qasm
// Before
qubit[2]q;
qubitq;

// After
qubit[2] q;
qubit q;
```

- No space between type and `[`
- Single space between `]` and identifier
- Single space between type and identifier (when no array)

### 3.2 Classical Bit Declaration

```qasm
// Before
bit[4]c;
bit c=0;

// After
bit[4] c;
bit c = 0;
```

- Same rules as qubit declaration
- Spaces around `=` for initialization

### 3.3 Constant/Variable Declaration

```qasm
// Before
const int[32]n=3;

// After
const int[32] n = 3;
```

---

## 4. Gate Calls

### 4.1 Basic Gates

```qasm
// Before
hq[0];

// After
h q[0];
```

- Single space between gate name and arguments

### 4.2 Multiple Arguments

```qasm
// Before
cxq[0],q[1];

// After
cx q[0], q[1];
```

- Single space after comma
- No space before comma

### 4.3 Parameterized Gates

```qasm
// Before
rz(pi/4)q[0];
cphase(pi/2)q[0],q[1];

// After
rz(pi / 4) q[0];
cphase(pi / 2) q[0], q[1];
```

- No space between gate name and `(`
- Spaces around operators inside parameters
- Single space between `)` and arguments

---

## 5. Operators

### 5.1 Arithmetic Operators

```qasm
// Before
pi/4+pi/8

// After
pi / 4 + pi / 8
```

- Spaces around `+`, `-`, `*`, `/`

### 5.2 Comparison Operators

```qasm
// Before
c==1
n!=0

// After
c == 1
n != 0
```

- Spaces around `==`, `!=`, `<`, `>`, `<=`, `>=`

### 5.3 Logical Operators

```qasm
// Before
a&&b
c||d

// After
a && b
c || d
```

### 5.4 Assignment Operator

```qasm
// Before
c=measure q;

// After
c = measure q;
```

---

## 6. Control Flow

### 6.1 If Statement

```qasm
// Before
if(c==1)x q;
if(c==1){x q;}

// After
if (c == 1) x q;
if (c == 1) { x q; }
```

- Single space between `if` and `(`
- Single space between `)` and statement/`{`

### 6.2 If-Else Statement

```qasm
if (c == 1) {
  x q;
} else {
  z q;
}
```

- Single space between `}` and `else`
- Single space between `else` and `{`

### 6.3 While Statement

```qasm
// Before
while(flags!=0){...}

// After
while (flags != 0) {
  ...
}
```

### 6.4 For Statement

```qasm
for int i in [0:n] {
  ...
}
```

---

## 7. Gate Definition

```qasm
// Before
gate mygate(theta)q{rz(theta)q;h q;}

// After
gate mygate(theta) q {
  rz(theta) q;
  h q;
}
```

- Single space between `gate` and gate name
- Single space between `)` and qubit arguments
- Single space between arguments and `{`
- Body indented by 2 spaces
- `}` on its own line

---

## 8. Subroutine Definition (def)

```qasm
// Before
def myfunc(qubit q)->bit{bit b;measure q->b;return b;}

// After
def myfunc(qubit q) -> bit {
  bit b;
  measure q -> b;
  return b;
}
```

- Spaces around `->`
- Body indented by 2 spaces

---

## 9. Measure Statement

### 9.1 Assignment Form

```qasm
// Before
c=measure q;

// After
c = measure q;
```

### 9.2 Arrow Form

```qasm
// Before
measure q->c;

// After
measure q -> c;
```

- Spaces around `->`

---

## 10. Barrier / Reset

```qasm
barrier q;
reset q;
```

- Single space between keyword and arguments

---

## 11. Indentation

- **Indent size**: 2 spaces (default)
- **Style**: Spaces only (no tabs)

```qasm
gate mygate q {
  h q;
  if (c == 1) {
    x q;
  }
}
```

---

## 12. Blank Lines

### 12.1 After Include

```qasm
OPENQASM 3.0;
include "stdgates.inc";

qubit q;
```

- One blank line after include block

### 12.2 Around Gate/Def Definitions

```qasm
qubit q;

gate mygate q {
  h q;
}

h q;
```

- One blank line before and after gate/def definitions

### 12.3 Consecutive Blank Lines

- Multiple consecutive blank lines are collapsed to one

---

## 13. Trailing

### 13.1 Trailing Whitespace

- Trailing whitespace on each line is removed

### 13.2 Trailing Newline

- Single newline at end of file (default)

---

## Rule Summary

| Category | Rule | Example |
|----------|------|---------|
| Spacing | After include | `include "file";` |
| Spacing | Between type and identifier | `qubit[2] q;` |
| Spacing | Between gate and arguments | `h q;` |
| Spacing | After comma | `cx q[0], q[1];` |
| Spacing | Around operators | `a + b`, `c == 1` |
| Spacing | Before braces | `{ ... }` |
| Spacing | After if/while | `if (cond)` |
| Indentation | Inside blocks | 2 spaces |
| Blank lines | After include | 1 line |
| Blank lines | Around gate/def | 1 line |
