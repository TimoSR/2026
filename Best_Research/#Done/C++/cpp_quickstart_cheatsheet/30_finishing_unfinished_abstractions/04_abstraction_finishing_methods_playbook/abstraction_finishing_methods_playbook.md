# abstraction_finishing_methods_playbook.cpp

## InDepth: Practical way to finish abstractions

When a technology feels unfinished for app usage, use this order:
1. add facade with domain-level methods
2. add default profiles for common intent
3. normalize errors
4. centralize lifecycle/ownership
5. add adapter boundary for external types

This turns low-level capability into project-ready usability.

The code models this as `CapabilityNeeds` + `recommendMethodsFromNeeds(...)`, so picks are derived from observed problems (error shape, lifecycle complexity, config flags) instead of hardcoded prose lists.
