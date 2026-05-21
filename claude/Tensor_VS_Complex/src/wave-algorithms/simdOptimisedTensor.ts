import type { Wave } from "../types";
import { optimizedTensorSimdWasmBytes } from "../wasm/optimizedTensorSimd.wasm-bytes";
import { optCombine } from "./optimisedTensor";

const MAX_WAVES = 16;
const FREQ_OFFSET_BYTES = 64;
const PHASE_OFFSET_BYTES = 128;

type SimdExports = {
  memory: WebAssembly.Memory;
  combine: (count: number, t: number) => number;
};

type SimdState =
  | {
      kind: "ready";
      exports: SimdExports;
      amp: Float32Array;
      freq: Float32Array;
      phase: Float32Array;
    }
  | { kind: "fallback" };

let state: SimdState | null = null;

export function simdOptCombine(waves: readonly Wave[], t: number): number {
  const simd = getSimdState();

  if (simd.kind !== "ready") {
    return optCombine(waves, t, 0.01);
  }

  const count = Math.min(waves.length, MAX_WAVES);
  const paddedCount = Math.ceil(count / 4) * 4;

  simd.amp.fill(0, 0, paddedCount);
  simd.freq.fill(0, 0, paddedCount);
  simd.phase.fill(0, 0, paddedCount);

  for (let index = 0; index < count; index += 1) {
    const wave = waves[index];
    simd.amp[index] = wave.amp;
    simd.freq[index] = wave.freq;
    simd.phase[index] = wave.phase;
  }

  return simd.exports.combine(paddedCount, t);
}

export function isSimdOptReady(): boolean {
  return getSimdState().kind === "ready";
}

function getSimdState(): SimdState {
  if (state) return state;

  if (
    typeof WebAssembly === "undefined" ||
    !WebAssembly.validate(optimizedTensorSimdWasmBytes)
  ) {
    state = { kind: "fallback" };
    return state;
  }

  try {
    const module = new WebAssembly.Module(optimizedTensorSimdWasmBytes);
    const instance = new WebAssembly.Instance(module, {});
    const exports = instance.exports as SimdExports;

    state = {
      kind: "ready",
      exports,
      amp: new Float32Array(exports.memory.buffer, 0, MAX_WAVES),
      freq: new Float32Array(exports.memory.buffer, FREQ_OFFSET_BYTES, MAX_WAVES),
      phase: new Float32Array(exports.memory.buffer, PHASE_OFFSET_BYTES, MAX_WAVES),
    };
  } catch {
    state = { kind: "fallback" };
  }

  return state;
}
