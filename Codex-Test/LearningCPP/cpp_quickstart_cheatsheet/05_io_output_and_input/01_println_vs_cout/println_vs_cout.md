# `println` vs `cout`

This section is intentionally split into two separate files (no preprocessors):

- `cout_style_output.cpp` (classic stream style)
- `println_style_cpp23.cpp` (modern C++23 print style)

## Pattern

`cout` file pattern:

```cpp
cout << "Name: " << name << "\n";
```

`println` file pattern (C++23):

```cpp
std::println("Name: {}", name);
```

## Why people like `println`
- cleaner formatting with `{}` placeholders
- often easier to scan for long output lines
- less `<<` chaining

## Why `cout` still appears everywhere
- supported in older standards/toolchains
- widely known and stable

## Practical rule
- If your compiler/toolchain supports C++23 print library: `println` is usually nicer.
- If not, use `cout` (or formatting library) without overthinking.

## InDepths

- `cout` chaining is universal and stable; `print`/`println` style improves readability for formatted output.
- For mixed toolchains, keep one clear style per file to reduce portability confusion.
- In production codebases, settle on one output style guideline and enforce consistency in reviews.

