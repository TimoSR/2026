# Header-Only Approach

Pattern:

```cpp
// everything lives in one header
class BankAccount { ... inline methods ... };
```

Upsides:
- Very easy to distribute (single header)
- No linker setup for a tiny library

Downsides:
- Can increase compile times in bigger projects
- Implementation details are exposed in headers

Use this for utility libraries, templates, or small reusable components.

## InDepths

- Header-only is convenient for distribution and templates, but increases include/compile pressure.
- Keep inline implementations short and stable to reduce rebuild churn.
- If compile time becomes painful, consider moving heavy logic to source files or modules.

