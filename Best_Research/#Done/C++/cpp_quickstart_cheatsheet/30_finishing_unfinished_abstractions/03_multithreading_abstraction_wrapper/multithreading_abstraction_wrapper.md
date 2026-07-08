# multithreading_abstraction_wrapper.cpp

## InDepth: What got improved

Raw threading code leaks:
- mutex/condition variable details
- lifecycle complexity
- task coordination logic

`SingleThreadTaskRunner` finishes that abstraction:
- caller only submits work
- result comes back as `future<string>`
- shutdown lifecycle is centralized

## InDepth: Why single worker first

A single worker thread already gives async behavior with simpler correctness than multi-worker pools.  
It is a safe first abstraction for many app workflows.
