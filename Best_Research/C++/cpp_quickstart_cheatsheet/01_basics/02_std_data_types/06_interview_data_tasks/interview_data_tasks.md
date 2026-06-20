# Interview Data Tasks

## Summary
- Practical transformations on text and numbers used in screening exercises.

## Syntax
```cpp
map<string, int> counts = frequencyByWord({"cpp", "docs", "cpp"});
vector<int> filtered = valuesAbove({10, 30, 5, 42}, 20);
pair<int, int> evenOdd = evenOddCount({1, 2, 3, 4});
tuple<int, int, int> top3 = top3OrZero({42, 7, 99, 18, 56});
```

## Complexity
- Frequency/count/filter passes: `O(N)`.
- `map` updates add `O(log K)` per insertion (`K` distinct keys).

## Memory
- `vector`/`map` own dynamic storage.
- tuple/pair outputs are value objects returned by copy elision/move.
