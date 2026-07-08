# Pattern: Class Templates

Class template pattern:

```cpp
template <typename T>
class Name { ... };
```

Use this when the data structure behavior is identical across types.

## InDepths

- Class templates are ideal for containers and wrappers.
- Keep template classes small and predictable at first.
- Prefer explicit class names (`Box<T>`, `SimpleStack<T>`) over clever abstractions.
