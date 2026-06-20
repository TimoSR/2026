# Common Data Structures in C++

## Summary
- Select containers by access pattern, ordering needs, and mutation behavior.

## Syntax
```cpp
vector<T>   // ordered dynamic list
map<K, V>   // sorted key-value map
set<T>      // sorted unique values
queue<T>    // FIFO adapter
```

## Operations
- `vector`: append, index, iterate.
- `map`/`set`: ordered lookup/insert/erase.
- `queue`: push back, pop front, inspect front.

## Complexity
- `vector` `push_back`: amortized `O(1)`.
- `vector` random access: `O(1)`.
- `vector` middle insert/erase: `O(N)`.
- `map`/`set` search/insert/erase: `O(log N)`.
- `queue` `push`/`pop`/`front`: `O(1)`.

## Memory
- `vector`: contiguous memory; growth can reallocate and move elements.
- `map`/`set`: node allocations with pointer overhead.
- `queue`: adapter over underlying container (commonly `deque`).

## Use When
- `vector` as default sequence container.
- `map`/`set` when sorted order or range queries matter.
- `queue` for strict FIFO workflows.

## Avoid When
- Using ordered containers without ordering requirements.
- Holding invalidated iterators after `vector` reallocation.

