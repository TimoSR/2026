# Async Programming in C++ (`future` / `async`)

Core pattern:

```cpp
future<int> f = async(launch::async, []() { return work(); });
int value = f.get();
```

Use this when:
- work is independent
- caller can do useful work before waiting

## InDepths

- `get()` blocks until result is ready; call it only when you actually need the value.
- Always decide explicitly if work should run truly async (`launch::async`).
- Async is about latency hiding and responsiveness, not automatic speedup in every case.
