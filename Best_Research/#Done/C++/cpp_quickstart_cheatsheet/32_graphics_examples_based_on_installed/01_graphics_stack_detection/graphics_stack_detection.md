# graphics_stack_detection.cpp

## InDepth: Why detect stack first

Graphics setup friction is mostly tooling and SDK availability.

Before writing rendering code:
1. detect what can run now
2. choose first backend
3. add missing build tools later

This keeps learning momentum high.

## What This Example Now Does

- checks real command availability on your current machine (`vulkaninfo`, `cargo`, `cmake`, `ninja`)
- checks Windows SDK presence from environment variables and common install path
- prints detected capability flags and then a recommended graphics learning order

Tests remain deterministic by using `detectGraphicsStackFromFlags(...)` with explicit input.
