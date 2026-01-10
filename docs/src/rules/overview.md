# Formatting Rules

## Spacing

### After Keywords

Single space after keywords:

```qasm
// Before
qubit[2]q;
include"file.qasm";

// After
qubit[2] q;
include "file.qasm";
```

### Around Operators

Spaces around binary operators:

```qasm
// Before
a+b*c

// After
a + b * c
```

### Gate Calls

```qasm
// Before
cxq[0],q[1];

// After
cx q[0], q[1];
```

## Indentation

Block contents are indented:

```qasm
gate mygate q {
    h q;
    x q;
}

if (c == 1) {
    x q;
}
```

## Blank Lines

- One blank line after `include` statements
- One blank line before/after gate definitions

```qasm
OPENQASM 3.0;
include "stdgates.qasm";

qubit q;

gate mygate q {
    h q;
}

h q;
```
