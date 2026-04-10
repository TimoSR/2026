# Optional and Parsing

## Summary
- Safe parse flow with `optional<T>` instead of magic error values.

## Syntax
```cpp
optional<int> parsed = tryParseInt("42");
int safe = valueOrDefault(parsed, -1);
```

## Complexity
- Parse by stream scanning: typically `O(N)` by text length.
- `value_or`: `O(1)`.

## Memory
- `optional<int>` stores value inline plus engaged/disengaged state.
