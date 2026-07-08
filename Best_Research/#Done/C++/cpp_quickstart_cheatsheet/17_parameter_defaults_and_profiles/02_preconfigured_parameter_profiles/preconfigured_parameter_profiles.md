# preconfigured_parameter_profiles.cpp

## InDepth: Why profiles are better than many booleans

If users must pass many flags, they usually do not know which combinations are safe.

Profiles solve this:
- pick an intent (`safe`, `fast`, `debug`)
- get predictable behavior
- reduce invalid combinations
- inferability matters: `inferProfileName(...)` should map options back to a clear intent name

## InDepth: Profile + override pattern

Common production pattern:
1. start from profile defaults
2. allow explicit overrides only where needed
