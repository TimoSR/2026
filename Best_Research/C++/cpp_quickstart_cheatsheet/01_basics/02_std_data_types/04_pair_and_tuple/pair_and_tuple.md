# Pair and Tuple

## Summary
- Grouped return values for 2 fields (`pair`) and 3+ fields (`tuple`).

## Syntax
```cpp
pair<string, int> userScore = userWithScore("Nora", 95);
tuple<string, int, bool> summary = accountSummary("Nora", 1200, true);
```

## Complexity
- Construction and access are `O(1)`.

## Memory
- `pair` and `tuple` store member values inline.
- Any `string` members can own heap memory.

## Practical Note
- Use named `struct` when public API readability matters more than compactness.
