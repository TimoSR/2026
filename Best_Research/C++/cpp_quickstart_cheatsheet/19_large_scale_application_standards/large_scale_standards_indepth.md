# Large-Scale Standards InDepth

This section focuses on standards used in very large C++ codebases.

## 1. Why Naming Standards Exist

At scale, naming is not style preference.
It is a navigation and communication tool.

A common Unreal-style mapping is:

- `A*` world actor style types
- `U*` managed object style types
- `I*` interfaces
- `F*` structs and plain value types
- `E*` enums
- `T*` templates

The main benefit is scan speed: type intent is visible from symbol names.

## 2. Prefixes Are Convention, Not Language Rules

C++ does not enforce these prefixes.
Teams enforce them through review, CI checks, and codegen/templates.

If your project is not Unreal-based, use only the parts that improve clarity.
Do not copy all prefixes blindly.

## 3. Large-App Standards Need Boundaries, Not Only Names

Naming alone does not scale architecture.
You also need dependency direction rules.

A common rule set:

- UI depends on Application
- Application depends on Domain
- Infrastructure adapts to Application/Domain contracts
- Domain does not depend on outer layers

## 4. Turn Standards Into Checks

Standards become real when they are executable:

- naming validation checks
- dependency rule checks
- package/module boundary checks

Manual policing does not scale to large teams.

## 5. Practical Adoption Strategy

1. Write standards in one short document.
2. Start with warnings in CI.
3. Fix high-churn modules first.
4. Promote warnings to required checks.
5. Keep exceptions explicit and temporary.

## 6. Unreal-Specific Note

Unreal naming conventions integrate with the engine ecosystem and reflection tooling.
When building Unreal projects, consistency with its conventions improves collaboration and discoverability.

For non-Unreal projects, copy the principle (predictable naming), not necessarily every exact prefix.
