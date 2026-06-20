# Platform-Specific Flow With `#if` / `#ifdef`

Core pattern:

```cpp
#if defined(_WIN32)
    // Windows code
#elif defined(__linux__)
    // Linux code
#else
    // fallback
#endif
```

Use this when:
- file paths or separators differ
- OS APIs differ
- default user directories differ

Keep this clean by:
- wrapping platform branches in small functions
- isolating platform code in one file/folder
- keeping most business logic platform-agnostic

## InDepths

- Keep platform branches thin and centralized; isolate OS specifics behind small wrapper functions.
- Avoid sprinkling `#if` across business logic files. Route platform differences through one adapter layer.
- Test one platform-agnostic path plus platform-specific paths separately where possible.
- Prefer examples that do real platform work: resolve config root, normalize path separators, and select native headers.

