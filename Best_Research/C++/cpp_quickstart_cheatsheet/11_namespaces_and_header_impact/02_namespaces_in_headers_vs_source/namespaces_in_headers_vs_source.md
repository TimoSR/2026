# namespaces_in_headers_vs_source.cpp

## InDepth: Why headers are higher risk

Headers are included by many files.  
Any `using namespace ...` in a header leaks names into every including translation unit.

That can cause:
- unexpected symbol collisions
- ambiguous calls
- harder-to-debug compile errors as project grows

## InDepth: Practical rule

- headers: always explicit namespaces and explicit `std::`
- source files: limited local `using` is acceptable when scoped and clear
- this example shows two real `parse` functions (`parse` and `analytics_headers::parse`) so explicit namespace qualification changes runtime behavior
