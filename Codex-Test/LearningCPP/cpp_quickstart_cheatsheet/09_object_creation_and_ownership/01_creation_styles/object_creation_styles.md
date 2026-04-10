# Pattern: Object Creation Options (Old and New)

You will see all four patterns in real C++ codebases:

```cpp
BankAccount account("Nora", 500.0);                 // stack object
auto account = new BankAccount("Nora", 500.0);      // raw heap (legacy)
auto account = make_unique<BankAccount>("Nora", 500.0); // modern single owner
shared_ptr<BankAccount> account = make_shared<BankAccount>("Nora", 500.0); // shared owner
weak_ptr<BankAccount> observer = account; // non-owning observer
```

Quick rule:
- Use stack objects by default.
- Use `make_unique` when lifetime must outlive current scope.
- Use `make_shared` when multiple owners are truly needed.
- Use `weak_ptr` to observe shared objects without keeping them alive.
- Avoid raw `new` in new code unless a low-level API requires it.

`weak_ptr` pattern:

```cpp
shared_ptr<BankAccount> sharedAccount = make_shared<BankAccount>("Nora", 500.0);
weak_ptr<BankAccount> weakAccount = sharedAccount;
shared_ptr<BankAccount> lockedAccount = weakAccount.lock();
```

- `lock()` returns a valid `shared_ptr` only if object still exists.
- `expired()` checks if object was already destroyed.

## InDepths

- Creation style is a lifetime decision first, syntax decision second.
- Stack objects are simplest and fastest for local scope lifetimes.
- Heap objects should have explicit ownership model (`unique_ptr` / `shared_ptr`) from the start.

