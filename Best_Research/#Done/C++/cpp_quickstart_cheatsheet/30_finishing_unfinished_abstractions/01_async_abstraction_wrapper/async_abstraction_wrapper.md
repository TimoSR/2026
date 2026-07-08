# async_abstraction_wrapper.cpp

## InDepth: What got improved

Raw async usage often leaks complexity:
- futures everywhere
- inconsistent timeout handling
- mixed error behavior

`AsyncTextService` finishes that abstraction by giving:
- a simple sync-style method (`runAndWait`)
- a timeout-normalized method (`runWithTimeout`)

## InDepth: Project pattern

Wrap low-level async primitives behind domain methods so app code reads as business behavior, not thread/future plumbing.
