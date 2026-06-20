# `cin` and Input Handling

Core beginner flow:

```cpp
int value;
cin >> value;
if (cin.fail()) {
    cin.clear();
    cin.ignore(...);
}
```

Why input handling matters:
- users type invalid input often
- failed stream state blocks further reads unless cleared

Useful pattern split:
- parse function (`tryReadInteger`)
- validation function (`isWithinRange`)
- flow function (`askAgeFlow`)

This keeps `cin` logic testable and less fragile.

## InDepths

- Stream failure state is sticky: once failed, reads keep failing until `clear()` is called.
- Always separate parse, validate, and business action. This makes interactive input testable and maintainable.
- For robust CLIs, prefer reading full lines then parsing, rather than direct token extraction everywhere.

