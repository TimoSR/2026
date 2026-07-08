# Text Basics

## Summary
- Text assembly, splitting, trimming, token replacement, and small text-focused functions.

## Syntax
```cpp
string fullName = buildFullName("Nora", "Jensen");
string email = buildEmailAddress("nora", "example.com");
string repeated = repeatText("C++", 3, " | ");
string rendered = renderTemplateAB("A, B", "House", "Number");
vector<string> parts = splitCommaSeparated("A, B, C");
vector<string> csv = splitCsvLine("red,green,blue");
```

## Complexity
- Split and trim operations are usually `O(N)`.
- Template replacement in this example is `O(N)` by template length.
- Repetition by count is `O(text length * repeatCount)`.

## Memory
- `string` owns character data.
- `vector<string>` owns dynamic storage for tokenized values.

## Practical Notes
- `splitCsvLine` in this example is intentionally simple and does not support quoted CSV rules.
- Keep string helpers small and composable: build, split, normalize, then render.
