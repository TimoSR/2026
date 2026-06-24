# Product UI Architecture

## Ownership

`src/ui` owns product-specific UI components. It composes `gui`, `graphics`,
and `diagnostics` into concrete screens and components.

Examples include performance metrics, graphics settings, pause menu, HUD, and
game menus.

## Rules

Product UI components may edit product state and settings models. They must not
call `Direct3DGraphics` methods directly.

For graphics options, a component edits `GraphicsSettings`. The application
coordinates the renderer update through `graphics.apply_settings(...)`.

The metrics component receives `PerformanceSample` and
`GraphicsPerformanceMetrics`, formats product-facing text, and emits generic
controls through the `gui` library.
