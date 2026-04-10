# html_parsing_basics.cpp

## InDepth: What this parser is and is not

This file shows a pattern-based parser for known/simple HTML fragments.

Good fit:
- controlled snippets
- learning string parsing flow
- tests that lock expected patterns

Not a full HTML parser:
- does not handle malformed HTML robustly
- does not decode entities
- does not support nested/complex edge cases

For production HTML parsing, use a dedicated parser library and keep this style only for lightweight extraction tasks.
