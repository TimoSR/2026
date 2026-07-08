# Pattern: List + Lookup Table

Use `vector<T>` when order matters and duplicates are fine.

Use `map<Key, Value>` when you need fast lookup by key.

Starter patterns:

```cpp
for (T item : items) { ... }
```

```cpp
mapData[key] += 1;
```

```cpp
for (pair<Key, Value> entry : mapData) { ... }
```

## InDepths

- Start by modeling behavior: sequence (`vector`) vs key lookup (`map`). Performance tuning comes after correctness.
- `map[key]` inserts missing keys; this is useful for counters but can surprise you in read-only paths.
- Use const iteration in read paths to prevent accidental mutation.

