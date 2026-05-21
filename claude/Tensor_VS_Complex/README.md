# Wave Simulation Benchmark

A React/Vite app for comparing four wave-combination implementations:

- Complex baseline
- Tensor-style typed buffer
- Binary lookup table
- Optimised tensor variant

## Run locally

```bash
pnpm install
pnpm dev
```

The dev server prints a local URL, usually `http://127.0.0.1:5173/`.

## Build

```bash
pnpm build
```

The production output is written to `dist/`.

## Typecheck

```bash
pnpm typecheck
```
