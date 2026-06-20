# Abstractions For Testing (C++ Interface Style)

C# interface idea maps to C++ abstract classes:

```cpp
class IThing {
public:
    virtual ~IThing() = default;
    virtual bool doWork() = 0;
};
```

Pattern for testable code:
- service depends on interface (`IEmailGateway&`)
- production uses real implementation
- tests inject fake/spy implementations

In this folder:
- `abstractions_with_shared_ptr.cpp` uses `shared_ptr<IEmailGateway>`
- `abstractions_with_reference.cpp` uses `IEmailGateway&`

`&` means reference in C++ (non-owning alias to an existing object).

Why this helps:
- deterministic tests
- no network/database side effects in tests
- easy behavior checks (success/failure/retry paths)

Common test doubles:
- Fake: simple working substitute
- Stub: returns fixed values
- Spy: records calls for assertions

## InDepths

- Abstractions are most valuable at boundaries (network, file system, clock, randomness).
- Use fake/spy doubles to make tests deterministic and focused on behavior.
- If abstraction count grows too fast, you may be abstracting too early; add interfaces at change-prone seams.

