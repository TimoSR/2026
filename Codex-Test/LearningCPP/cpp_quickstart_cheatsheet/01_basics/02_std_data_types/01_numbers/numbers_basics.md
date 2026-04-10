# Numbers Basics

## Summary
- Integer bounds, aggregation, and min/max operations.

## Syntax
```cpp
int age = cappedAge(150);
long long total = sumValues({10, 20, 30});
double average = averageValues({10, 20, 30});
pair<int, int> minMax = minMaxValues({95, 88, 77, 100});
```

## Complexity
- `cappedAge`: `O(1)`.
- `sumValues`: `O(N)`.
- `averageValues`: `O(N)`.
- `minMaxValues`: `O(N)`.

## Memory
- Input vector owns dynamic storage.
- Local counters and return primitives are stack values.
