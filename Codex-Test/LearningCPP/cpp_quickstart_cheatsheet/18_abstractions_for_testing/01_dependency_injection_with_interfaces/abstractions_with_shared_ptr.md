# abstractions_with_shared_ptr

`abstractions_with_shared_ptr.cpp` shows dependency injection using `shared_ptr<Interface>`.

Use this variant when:
- multiple services share the same test double/implementation
- lifetime should be reference-counted across owners

Tradeoff:
- easier wiring in tests and composition roots
- ownership graph becomes less explicit than reference injection

See also:
- `abstractions_with_reference.cpp` for non-owning injection
- `abstractions_for_testing.md` for broader testing patterns

