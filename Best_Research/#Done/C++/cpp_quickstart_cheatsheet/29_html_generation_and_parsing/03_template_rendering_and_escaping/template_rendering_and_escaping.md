# template_rendering_and_escaping.cpp

## InDepth: Replacement pattern for dynamic content

`renderTemplate(...)` uses a common placeholder pattern:
- template text contains keys like `{{name}}`
- C++ map provides key/value data
- each placeholder is replaced with data

This pattern keeps HTML structure stable and data injection explicit.

## InDepth: Safe vs unsafe dynamic values

When values come from users, always escape before rendering.

In this file:
- `buildSafeWelcomePageHtml(...)` escapes values first
- rendered HTML keeps tags in template only
- user values stay as text, not executable markup
