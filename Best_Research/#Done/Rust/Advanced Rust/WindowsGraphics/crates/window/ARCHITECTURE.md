# Window Library Architecture

## Ownership

The `window` library owns Win32 window creation, native message processing,
window lifetime, and native input collection.

It does not own GUI layout, widgets, renderer settings, scene rendering, or
product-specific input behavior.

## GUI Boundary

The library will expose native events as an input snapshot suitable for the
generic `gui` library. The input snapshot will eventually include pointer
position, pressed and released buttons, keyboard keys, typed characters, and
scroll input.

The `window` library must not import `gui`, `graphics`, `diagnostics`, or
`src/ui`. Dependencies flow from product code toward this library, never the
other way around.
