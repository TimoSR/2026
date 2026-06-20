# Header + Source Split (Traditional)

Pattern:

```cpp
// .h = declarations
class BankAccount { ... };

// .cpp = implementations
BankAccount::BankAccount(...) { ... }
```

Upsides:
- Clear API boundary (what exists vs how it works)
- Faster incremental builds in larger projects
- Easier teamwork on larger codebases

Downsides:
- More files and navigation overhead
- Harder for beginners to scan quickly
- Build setup is slightly more complex

Use this when code grows beyond small learning exercises.

## InDepths

- Header/source split clarifies API boundaries and reduces unnecessary recompilation in larger projects.
- Keep headers minimal (declarations only) to avoid dependency cascades.
- Favor forward declarations where practical to shrink compile surfaces.

