# Composition Over Inheritance (With Inheritance Comparison)

Core idea:
- Inheritance reuses via class hierarchy.
- Composition reuses via pluggable collaborators.

Why composition is often better:
- easier to swap behavior at runtime
- lower coupling between types
- usually easier to test with fakes/stubs

Inheritance is still useful:
- true "is-a" polymorphic contracts
- shared abstraction boundaries (not just code reuse)

## InDepths

- Use inheritance for stable abstractions; use composition for flexible behavior wiring.
- If you need many inheritance subclasses for combinations, composition is usually the better design.
- Composition scales better with feature growth because behaviors can be mixed without hierarchy explosion.
