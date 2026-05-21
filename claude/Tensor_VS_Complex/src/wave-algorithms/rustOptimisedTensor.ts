import type { Wave } from "../types";
import { rustOptimizedTensorWasmBytes } from "../wasm/rustOptimizedTensor.wasm-bytes";
import { optCombine } from "./optimisedTensor";

const FALLBACK_MAX_WAVES = 16;
const STEP = 0.01;

type RustExports = {
  memory: WebAssembly.Memory;
  max_waves: () => number;
  amps_ptr: () => number;
  freqs_ptr: () => number;
  phases_ptr: () => number;
  combine: (count: number, t: number, step: number) => number;
};

type RustState =
  | {
      kind: "ready";
      exports: RustExports;
      amp: Float32Array;
      freq: Float32Array;
      phase: Float32Array;
      maxWaves: number;
    }
  | { kind: "fallback" };

let state: RustState | null = null;

export function rustOptCombine(waves: readonly Wave[], t: number): number {
  const rust = getRustState();

  if (rust.kind !== "ready") {
    return optCombine(waves, t, STEP);
  }

  const count = Math.min(waves.length, rust.maxWaves);

  for (let index = 0; index < count; index += 1) {
    const wave = waves[index];
    rust.amp[index] = wave.amp;
    rust.freq[index] = wave.freq;
    rust.phase[index] = wave.phase;
  }

  return rust.exports.combine(count, t, STEP);
}

export function isRustOptReady(): boolean {
  return getRustState().kind === "ready";
}

function getRustState(): RustState {
  if (state) return state;

  if (
    typeof WebAssembly === "undefined" ||
    !WebAssembly.validate(rustOptimizedTensorWasmBytes)
  ) {
    state = { kind: "fallback" };
    return state;
  }

  try {
    const module = new WebAssembly.Module(rustOptimizedTensorWasmBytes);
    const instance = new WebAssembly.Instance(module, {});
    const exports = instance.exports as RustExports;
    const maxWaves = Math.min(exports.max_waves(), FALLBACK_MAX_WAVES);

    state = {
      kind: "ready",
      exports,
      maxWaves,
      amp: new Float32Array(exports.memory.buffer, exports.amps_ptr(), maxWaves),
      freq: new Float32Array(exports.memory.buffer, exports.freqs_ptr(), maxWaves),
      phase: new Float32Array(exports.memory.buffer, exports.phases_ptr(), maxWaves),
    };
  } catch {
    state = { kind: "fallback" };
  }

  return state;
}
