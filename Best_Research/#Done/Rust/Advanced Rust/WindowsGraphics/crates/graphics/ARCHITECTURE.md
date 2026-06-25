# Graphics Library Architecture

## Ownership

The `graphics` library owns scene rendering, GPU resources, renderer settings,
and execution of generic UI draw data.

It does not own product screens, performance-metrics formatting, menus, HUD
state, or GUI widget behavior.

## Graphics Settings

`GraphicsSettings` describes renderer capabilities. Product UI may edit a
`GraphicsSettings` value, but it must not call renderer methods directly.

The product applies the desired settings through the single public boundary:

```rust
graphics.apply_settings(&graphics_settings)?;
```

Individual renderer setters remain private implementation details so that
renderer calls do not spread through UI components.

## Antialiasing Boundary

MSAA and TAA are scene-rendering settings. The renderer draws UI after the
scene has been resolved, so scene antialiasing does not improve GUI text or GUI
edge quality.

GUI quality needs a separate path: texture-backed font atlas, alpha blending,
clip rectangles, and UI draw commands. DirectWrite through `windows-rs` is a
suitable future glyph source without adding a dependency.
