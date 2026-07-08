# abstractions_with_reference

`abstractions_with_reference.cpp` shows dependency injection using references (`Interface&`).

Use this variant when:
- dependency lifetime is managed elsewhere
- service cannot exist without a valid dependency
- you want non-null semantics by construction

See also:
- `abstractions_with_shared_ptr.cpp` for shared ownership-based injection
- `abstractions_for_testing.md` for comparison guidance

