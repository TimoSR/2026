# html_generation_basics.cpp

## InDepth: Why escape text?

If user data is inserted directly into HTML, characters like `<` and `&` can break markup or introduce script injection risks.

Pattern:
1. Keep data as plain text in C++.
2. Escape it at output boundaries (`escapeHtmlText(...)`).
3. Build markup from escaped pieces.

## InDepth: When this style is enough

This string-based approach is useful for:
- tiny tools
- server-side generated snippets
- tests that verify output shape

For large views, move to a template engine and keep the same boundary rule: escape user text before rendering.
