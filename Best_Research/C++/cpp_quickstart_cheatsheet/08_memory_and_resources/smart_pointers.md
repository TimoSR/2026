# Pattern: Ownership by Type

`unique_ptr<T>`:
- exactly one owner
- move ownership when needed

`shared_ptr<T>`:
- multiple owners
- object lives until last owner is gone

Starter rule:
- default to stack values
- use smart pointers only when object lifetime must outlive one scope or be shared

## InDepths

- Ownership is the core question: who is responsible for lifetime?
- `unique_ptr` is the safest default for heap ownership; move ownership explicitly when needed.
- `shared_ptr` is powerful but easy to overuse; introduce it only when multiple real owners exist.

