# Attributes: What They Are

Attributes are compiler-facing hints/metadata.

Pattern:

```cpp
[[attribute_name]]
```

Examples used here:
- `[[deprecated("message")]]`: warns when old API is used
- `[[maybe_unused]]`: avoids unused warnings for intentional cases

When valid:
- migration paths (`deprecated`)
- conditional logging/debug paths (`maybe_unused`)

When overkill:
- adding attributes everywhere without real warning/noise problem

Practical rule:
- add attributes to solve a real warning or migration need
- avoid attribute spam

## InDepths

- Attributes are best treated as intent markers for compilers and readers, not decoration.
- `deprecated` is a migration tool: keep message text concrete with replacement guidance.
- Add attributes only where they remove real ambiguity/warnings; avoid blanket usage.

