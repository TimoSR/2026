import type { Wave } from "../types";
import { cOptimizedTensorWasmBytes } from "../wasm/cOptimizedTensor.wasm-bytes";
import { optCombine } from "./optimisedTensor";

const FALLBACK_MAX_WAVES = 16;
const STEP = 0.01;

type CExports = {
  memory: WebAssembly.Memory;
  max_waves: () => number;
  amps_ptr: () => number;
  freqs_ptr: () => number;
  phases_ptr: () => number;
  combine: (count: number, t: number, step: number) => number;
};

type CState =
  | {
      kind: "ready";
      exports: CExports;
      amp: Float32Array;
      freq: Float32Array;
      phase: Float32Array;
      maxWaves: number;
    }
  | { kind: "fallback" };

let state: CState | null = null;

export function cOptCombine(waves: readonly Wave[], t: number): number {
  const c = getCState();

  if (c.kind !== "ready") {
    return optCombine(waves, t, STEP);
  }

  const count = Math.min(waves.length, c.maxWaves);

  for (let index = 0; index < count; index += 1) {
    const wave = waves[index];
    c.amp[index] = wave.amp;
    c.freq[index] = wave.freq;
    c.phase[index] = wave.phase;
  }

  return c.exports.combine(count, t, STEP);
}

export function isCOptReady(): boolean {
  return getCState().kind === "ready";
}

function getCState(): CState {
  if (state) return state;

  if (
    typeof WebAssembly === "undefined" ||
    !WebAssembly.validate(cOptimizedTensorWasmBytes)
  ) {
    state = { kind: "fallback" };
    return state;
  }

  try {
    const module = new WebAssembly.Module(cOptimizedTensorWasmBytes);
    const instance = new WebAssembly.Instance(module, {});
    const exports = instance.exports as CExports;
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
