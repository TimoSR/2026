# My Widget Project

A small production-style TypeScript project for an embeddable floating widget.

It contains:

```txt
packages/widget  -> framework-agnostic widget package
apps/demo        -> Vite demo page that mounts the widget
```

The widget is mounted into Shadow DOM, so its CSS is isolated from the host page.

## Requirements

- Node.js 20 LTS or newer recommended
- npm 10 or newer recommended
- Windows PowerShell, Command Prompt, Git Bash, macOS, or Linux shell

This version does **not** use npm workspaces and does **not** use the `workspace:*` protocol.

## Run the demo

From the project root:

```bash
npm install
npm run dev
```

Then open:

```txt
http://127.0.0.1:5173
```

You should see a demo page with a floating AZTRO widget button in the bottom-right corner.

## Build everything

```bash
npm run build
```

This runs:

```bash
npm run build:widget
npm run build:demo
```

The widget package output is written to:

```txt
packages/widget/dist
```

The demo build output is written to:

```txt
apps/demo/dist
```

## Type-check

```bash
npm run typecheck
```

## Project structure

```txt
my-widget-project/
├─ package.json
├─ README.md
├─ packages/
│  └─ widget/
│     ├─ package.json
│     ├─ tsconfig.json
│     ├─ tsup.config.ts
│     └─ src/
│        ├─ dom.ts
│        ├─ index.ts
│        ├─ mount.ts
│        ├─ options.ts
│        └─ styles.ts
└─ apps/
   └─ demo/
      ├─ index.html
      ├─ package.json
      ├─ tsconfig.json
      ├─ vite.config.ts
      └─ src/
         ├─ main.ts
         └─ page.css
```

## How the demo mounts the widget

`apps/demo/src/main.ts`:

```ts
import { mountWidget } from "@aztro/widget";

import "./page.css";

const widget = mountWidget({
  appId: "demo-app",
  title: "AZTRO Tool",
  subtitle: "Demo embedded widget",
  position: "bottom-right",
  theme: "system",
  initiallyOpen: false,
  width: 440,
  height: 600
});

window.addEventListener("aztro-widget:action", (event) => {
  if (!(event instanceof CustomEvent)) {
    return;
  }

  console.log("Widget action received:", event.detail);
});

window.addEventListener("beforeunload", () => {
  widget.destroy();
});
```

## Why the demo uses a Vite alias

The demo imports the local widget source through this alias:

```ts
"@aztro/widget": "../../packages/widget/src/index.ts"
```

That means you can run this repository with a plain root-level `npm install`, without npm workspaces, pnpm, yarn, or `workspace:*` support.

The package itself can still be built as a standalone npm package using:

```bash
npm run build:widget
```

## If you previously extracted an older ZIP

Delete the old extracted folder completely before extracting this version again.

On PowerShell:

```powershell
cd C:\Users\timot\code\2026\Research
Remove-Item -Recurse -Force .\my-widget-project
```

Then extract the new ZIP and run:

```powershell
cd .\my-widget-project
npm install
npm run dev
```

If you still see this error:

```txt
Unsupported URL Type "workspace:": workspace:*
```

then you are not running this corrected version. Search the folder:

```powershell
Select-String -Path .\**\* -Pattern "workspace:" -Recurse
```

There should be no `workspace:*` value in any `package.json`.

## Publishing later

When you want to publish the widget package, build it first:

```bash
npm run build:widget
```

Then publish from the package folder:

```bash
cd packages/widget
npm publish --access public
```

For private packages, configure your npm scope and registry before publishing.
