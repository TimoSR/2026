# Threading Basics in C++

Starter building blocks:
- `std::thread`
- `std::mutex` + `std::lock_guard`
- `std::atomic<T>`

Pattern:

```cpp
thread worker([]() { /* work */ });
worker.join();
```

`mutex` protects shared mutable state.  
`atomic` is useful for simple lock-free counters/flags.

## InDepths

- Always define ownership of shared state before adding threads.
- Prefer message passing or local state where possible; shared mutation is the risky part.
- Join all threads deterministically to avoid lifetime bugs.
