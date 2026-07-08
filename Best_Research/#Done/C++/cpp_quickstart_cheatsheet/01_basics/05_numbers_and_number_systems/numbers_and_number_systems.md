# Numeric Types and Number Systems

## Summary
- Select numeric types by range and precision, and literal format by readability/context.

## Syntax
```cpp
int count = 42;
long long total = 9000000000;
double ratio = 0.75;

0x2A;     // hexadecimal
052;      // octal
0b101010; // binary
```

## Operations
- Constant-time arithmetic for primitive numeric types.
- Convert between text and numeric values when needed.
- Validate literal equivalence directly (for example `areEquivalentNumberLiterals()` for decimal/hex/octal/binary forms).

## Complexity
- Arithmetic operations: `O(1)`.
- Parse/format from text: `O(N)` by text length.

## Memory and Representation
- Local numeric objects are typically stack-allocated.
- Typical sizes (platform-dependent): `int` 4 bytes, `long long` 8 bytes, `double` 8 bytes.
- `double` is binary floating-point; some decimal values are non-exact.

## Use When
- `int` for default counters and small ranges.
- `long long` for larger integer ranges.
- Fixed-width integers for protocol/file schema stability.

## Avoid When
- Floating-point equality checks use exact `==` in numeric logic.
- Integer ranges are assumed without overflow analysis.

