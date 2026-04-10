# Modern C++ cheat sheet

Pattern-first.
Example-first.
Small files.
Fast scanning.

## Suggested reading order

1. `01-basics`
2. `02-types-and-modeling`
3. `03-data-and-iteration`
4. `04-functions-and-interfaces`
5. `05-ownership-and-lifetime`
6. `06-classes`
7. `07-standard-library`
8. `08-errors`
9. `09-concurrency`

## Working defaults

```cpp
#include <string>
#include <vector>
#include <optional>
#include <expected>
#include <span>
#include <ranges>
#include <memory>
#include <format>
#include <filesystem>
#include <chrono>

using namespace std;
```

## Core habits

```cpp
// prefer values
// prefer string, string_view, vector, span
// return values directly
// keep classes valid by construction
// use unique_ptr only when ownership really needs indirection
// use optional / expected / variant to model outcomes
// use algorithms and ranges when they make intent clearer
```
