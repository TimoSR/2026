# Wave Simulation Benchmark

A React/Vite app for comparing wave-combination implementations:

- Complex baseline
- Tensor-style typed buffer
- Binary lookup table
- Optimised tensor variant
- C compiled-to-WebAssembly variant
- Rust compiled-to-WebAssembly variant

## Run locally

```bash
pnpm install
pnpm dev
```

The dev server prints a local URL, usually `http://127.0.0.1:5173/`.

The Rust implementation is compiled during `pnpm dev`, `pnpm build`, and
`pnpm typecheck`. If the wasm target is not installed yet, run:

```bash
rustup target add wasm32-unknown-unknown
```

## Native Rust example

Run the optimized tensor code directly as Rust, without compiling it to
WebAssembly:

```bash
pnpm rust:direct
```

That command runs `rust/optimized-tensor/examples/direct_call.rs`, which calls
the Rust `combine_optimized_tensor` function directly.

## Build

```bash
pnpm build
```

The production output is written to `dist/`.

## Typecheck

```bash
pnpm typecheck
```
