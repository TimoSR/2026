# Visual HTML Component Builder Widget

This folder shows the shape for a blank website that installs a visual HTML component builder widget. There is no TanStack Router, no routes, and no app framework coupling beyond React.

The current widget starts with the simplest builder feature:

- A bottom-right launcher button.
- A full-screen blank website canvas where the user can drag in multiple `div` elements from the widget.
- Direct movement and bottom-right resize on the selected page div.
- An element list in the widget for selecting which div is active.
- Multi-select with `Ctrl`/`Cmd`/`Shift` click on canvas elements or list rows.
- Duplicate the current selection with `Ctrl+D` / `Cmd+D` or the Duplicate button.
- Layer nesting: select multiple elements, then use **Nest selected**. The first selected element becomes the parent, and the rest become children positioned relative to it.
- Photoshop-style layer manager controls for selected layers: move up, move down, hide/show, nest, and unnest.
- Drop an image file onto a div to create an `img` child inside that div.
- A color picker for the visual base color.
- Optional grid mode, enabled by default, where the div edges snap to the grid.
- A Save button and `Ctrl+S` shortcut with visible saved feedback.
- Automatic local persistence so positions and sizes are restored after reloads.
- Live generated HTML/CSS that updates when any div is placed, moved, resized, recolored, or selected.

The package-facing API lives in `src/widget/index.ts`. A consuming app should be able to use it like this:

```tsx
import {
  ComponentBuilderCanvas,
  ComponentBuilderWidget,
  ComponentBuilderDevtools,
  componentBuilderPlugin,
} from '@your-scope/component-builder-widget'
```

For a normal blank React website, install the widget and render it. The widget only appears as a bottom-right launcher button until the user opens it:

```tsx
import {
  ComponentBuilderCanvas,
  ComponentBuilderWidget,
  ComponentBuilderDevtools,
} from '@your-scope/component-builder-widget'

export function App() {
  return (
    <>
      <main>
        <ComponentBuilderCanvas />
      </main>
      <ComponentBuilderWidget />
      <ComponentBuilderDevtools />
    </>
  )
}
```

If an app already owns the TanStack Devtools shell, install only the plugin:

```tsx
import { TanStackDevtools } from '@tanstack/react-devtools'
import { componentBuilderPlugin } from '@your-scope/component-builder-widget'

export function AppDevtools() {
  return <TanStackDevtools plugins={[componentBuilderPlugin]} />
}
```

Run it:

```bash
npm install
npm run dev
```

The custom widget state lives in `src/widget/researchWidgetStore.ts`. Every state mutation emits a typed `widget-state` event through `src/widget/researchWidgetEventClient.ts`, and `src/widget/ResearchWidgetDevtools.tsx` subscribes to those events to keep the devtools panel live.

`vite.config.ts` sets `removeDevtoolsOnBuild: false` so this demo keeps the devtools visible even when you run `npm run build`.

Docs used:

- https://tanstack.com/devtools/latest/docs/framework/react/guides/custom-plugins
- https://tanstack.com/devtools/latest/docs/quick-start
