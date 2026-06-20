# convention_checks_and_architecture_guardrails.cpp

## InDepth

Large applications usually enforce standards automatically.

This example shows two practical checks:

- naming guardrails (prefix-based categories)
- dependency direction guardrails (which layer may call which layer), with executable checks via `architectureViolationCount(...)`

Without automated checks, standards drift as teams and modules grow.
