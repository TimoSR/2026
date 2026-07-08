# Why `const` Is Everywhere

`const` is common because it helps express intent and prevent accidental mutation.

Valid reasons:
- safer APIs (`const` methods and `const&` parameters)
- fewer accidental edits to values
- clearer read-only boundaries in teams

Can it be overused?
- yes, if every tiny local variable becomes `const` and hurts readability

Balanced rule:
- always use `const` for read-only parameters and read-only methods
- use `const` for important local values that should not change
- skip `const` when it adds noise and no real safety benefit

Short answer: reason is valid, but style should stay practical.

## InDepths

- `const` on APIs (parameters/methods) is high-value because it communicates mutation guarantees across modules.
- Over-const local variables can reduce readability if every line becomes type noise.
- Good balance: strong const at boundaries, pragmatic const inside short local logic.

