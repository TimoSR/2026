# module_boundaries_and_ownership.cpp

## InDepth

Large codebases usually standardize around boundaries:

- application service depends on interfaces (`I*`)
- infrastructure implements those interfaces (`FInMemory*`, database adapters, API clients)
- business flow lives in one service, not spread across storage/network classes

This pattern keeps modules replaceable and test-friendly as the codebase grows.
