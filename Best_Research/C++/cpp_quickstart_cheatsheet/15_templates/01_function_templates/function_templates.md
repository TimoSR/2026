# Pattern: Function Templates (C++ Generics)

Function template pattern:

```cpp
template <typename T>
T functionName(T value) { ... }
```

This is C++ generic programming for reusable logic across types.

## InDepths

- Start with templates only when the behavior is truly type-generic.
- If logic is domain-specific, regular functions are usually clearer.
- Template errors can be noisy; keep template functions small and focused.
