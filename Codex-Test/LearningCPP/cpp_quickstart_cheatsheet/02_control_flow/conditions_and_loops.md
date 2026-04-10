# Pattern: Decide, Branch, Then Repeat

Decision pattern:

```cpp
if (condition) { return "A"; }
if (otherCondition) { return "B"; }
return "fallback";
```

Loop pattern:

```cpp
for (Type item : items) {
    if (match) { return item; }
}
return fallback;
```

Switch pattern:

```cpp
switch (value) {
case 1: return "One";
case 2: return "Two";
default: return "Other";
}
```

Modern concise pattern:

```cpp
if (auto found = find(items.begin(), items.end(), target); found != items.end()) {
    return true;
}
return false;
```

Use this for:
- categorizing values
- validating input
- finding first matching item
- command routing with `switch`
- skipping/short-circuiting with `continue` and `break`

## InDepths

- Order your `if` branches from most specific to most general to avoid hidden shadow-branch bugs.
- In loops, return early when goal is "find first match". It is clearer than tracking manual state flags.
- Keep each branch action small. Big branch bodies are a refactor signal toward helper functions.
- Use `switch` when multiple discrete values map to fixed outcomes.
- Use `while` when termination depends on changing runtime state.
- Use `do-while` when at least one iteration must happen (for example login or retry flows).

