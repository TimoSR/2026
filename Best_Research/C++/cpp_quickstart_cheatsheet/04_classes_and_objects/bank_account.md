# Pattern: Bundle Data + Rules

Class pattern:

```cpp
class Name {
private:
    // data
public:
    // constructor
    // methods
};
```

Why this is useful:
- keeps related data and behavior together
- protects internal state with private fields
- methods become the safe API (`deposit`, `withdraw`)

## InDepths

- Class methods should encode business rules (`withdraw` rules) so invalid state transitions are impossible from outside.
- Keep fields private and expose behavior-focused methods; this preserves invariants over time.
- If a method both validates and mutates state, tests should cover success and rejection paths.

