# namespace_aliases_and_layout.cpp

## InDepth: Alias tradeoff

Namespace aliases reduce visual noise in source files with deep module paths.

Good:
- local alias in `.cpp` files
- keeps implementation readable
- keeps loop-heavy code readable (see batch subject generation)

Risk:
- putting broad aliases in headers can leak naming choices to consumers

Use alias where it helps readability, but keep public headers explicit.
