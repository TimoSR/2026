# `#define`: When To Use It, When Not To

Pattern examples:

```cpp
#define APP_NAME "CPP Quickstart"          // often okay for tiny constants
#define BAD_SQUARE(x) x * x                 // dangerous macro style
```

Use `#define` for:
- compile-time flags (`#ifdef FEATURE_FLAG`)
- include guards
- platform switches

Avoid `#define` for normal values/functions when possible.
Prefer:
- `constexpr` values
- `const` values
- normal functions / lambdas / templates

Why macros can be bad:
- no type checking
- debugger view is weaker
- expression expansion bugs (operator precedence)

Example bug:

```cpp
BAD_SQUARE(2 + 3) // becomes 2 + 3 * 2 + 3 => 11
```

## InDepths

- Macros are text substitution, not language-level symbols. They bypass type safety and scoping.
- Use macros for compile-time selection and guards; avoid them for routine value/function logic.
- If a macro has arguments, parenthesize aggressively or replace with `constexpr`/functions.

