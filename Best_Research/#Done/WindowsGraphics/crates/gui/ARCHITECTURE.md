# GUI Library Architecture

## Ownership

The `gui` library is a reusable UI foundation. It owns generic controls,
retained box layout, widget interaction state, and UI draw data.

It does not own performance metrics, antialiasing settings, pause menus, HUD
state, renderer instances, diagnostics, or any other product concept.

## Public API Rules

The public API must remain control-oriented and human-friendly.

```rust
user_interface.begin_panel("Graphics settings");

user_interface.checkbox(
    "Temporal antialiasing",
    &mut graphics_settings.is_temporal_antialiasing_enabled,
);

user_interface.end_panel();
```

Widget identifiers are internal implementation details. Product code must not
create public `UiId` values or pass ImGui-style hidden-label strings.

The GUI derives internal identifiers from panel scope, widget label, sibling
occurrence, and, when useful, bound state.

## Retained Layout Tree

The GUI has a retained `UserInterfaceLayout` tree for human-planned screens.
It is made of `UserInterfaceBox` objects. A box can have child boxes, and a
child is resolved relative to its parent, so it follows the parent when the
screen is resized, moved, or zoomed.

Layout input is screen-relative rather than pixel-relative. The root boxes use
the screen as their parent; all other boxes use their immediate parent. Pixels
are calculated only while producing draw data and handling input. Immediate
text panels use the same screen-relative placement convention.

Child boxes can be freeform, horizontally glued, or vertically glued. Glued
layouts are intended for bars, sidebars, split panes, and similar groups where
adjacent edges must remain connected.

Each box has an explicit `UserInterfaceLayer`. Boxes render from lower to
higher layers, like Photoshop layers. A child may use any layer, allowing an
overlay or modal child to render above its parent.

`UserInterfaceLayout` owns its zoom factor and screen-relative zoom centre.
Changing either changes the resolved layout without rewriting any box bounds.

The retained tree is intentionally not a CSS parser, DOM implementation, or
general-purpose scene framework. It provides only the concrete capabilities
required for planning and switching game UI layouts.

```rust
let mut toolbar = UserInterfaceBox::new(UserInterfaceBoxLayout {
    relative_bounds: UserInterfaceRelativeRectangle::new(0.05, 0.05, 0.90, 0.10),
    children_layout: UserInterfaceChildrenLayout::Horizontal,
    ..Default::default()
});

toolbar.add_child(UserInterfaceBox::new(UserInterfaceBoxLayout {
    layout_weight: 1.0,
    ..Default::default()
}));
toolbar.add_child(UserInterfaceBox::new(UserInterfaceBoxLayout {
    layout_weight: 3.0,
    ..Default::default()
}));

let mut layout = UserInterfaceLayout::new();
layout.add_box(toolbar);
layout.set_zoom_factor(1.25);

user_interface.begin_frame();
user_interface.add_layout(&layout);
```

The two children remain attached, use one-quarter and three-quarters of the
toolbar, and continue to scale with the screen.

## Extension Order

1. Input snapshot and hit testing against resolved box bounds.
2. Padding, margin, alignment, and flexible child sizing where a product needs
   them.
3. Hover, active, focus, scroll, and internal widget state.
4. Label, button, checkbox, slider, separator, panel, and scroll area.
5. Draw commands with vertices, indices, clipping, texture/font bindings, and
   layer ordering.

Use CSS-inspired box and flex concepts where useful. Do not add a CSS parser,
selector engine, or general-purpose framework without a concrete product
requirement.
