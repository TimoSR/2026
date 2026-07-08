# Values and Output

## Summary
- Defines local values and writes formatted text to output streams.

## Syntax
```cpp
Type variableName = value;
cout << "Label: " << variableName << "\n";
```

## Operations
- Initialize variable from literal or expression.
- Stream values with `<<`.
- End line with `"\n"` (no forced flush) or `std::endl` (flushes).

## Complexity
- Primitive assignment: `O(1)`.
- Stream output: `O(N)` by emitted character count.

## Memory
- Local primitives: typically stack.
- `string`: owns character storage; may allocate heap memory.

## Use When
- You need low-friction diagnostics or CLI output.
- You want explicit control over output order.

## Avoid When
- High-frequency logging in hot paths without buffering strategy.
- You need structured logs (JSON/fields) but only print raw text.

