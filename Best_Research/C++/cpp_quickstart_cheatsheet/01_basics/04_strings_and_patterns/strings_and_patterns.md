# Strings, Templates, and Replacements

## Summary
- Builds, transforms, and renders text with explicit ownership and replacement rules.

## Syntax
```cpp
"A" + ", " + "B"
replaceAll(text, "A", "X")
renderTemplate("Hi {name}")
```

## Operations
- Concatenate fragments.
- Replace tokens/substrings.
- Render template placeholders with provided values.

## Complexity
- Single scan/replace: usually `O(N)`.
- Repeated full-string replace loops: up to `O(N * replacements)`.

## Memory
- `string` owns mutable storage and may reallocate on growth.
- `string_view` does not own memory; lifetime must outlive the view.

## Use When
- Output format is textual and token-driven.
- Placeholder contract is fixed and validated.

## Avoid When
- Unescaped user values are injected into HTML/SQL/commands.
- Hot loops build large strings via repeated `+` without reserve/buffer strategy.

