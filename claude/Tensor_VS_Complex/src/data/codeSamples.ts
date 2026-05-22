import type { AlgorithmKey } from "../types";
import binaryCode from "../wave-algorithms/binary.ts?raw";
import complexCode from "../wave-algorithms/complex.ts?raw";
import optimisedTensorCode from "../wave-algorithms/optimisedTensor.ts?raw";
import rustNativeCode from "../../rust/optimized-tensor/examples/direct_call.rs?raw";
import rustOptimisedTensorCode from "../../rust/optimized-tensor/src/lib.rs?raw";
import simdOptimisedTensorCode from "../wave-algorithms/simdOptimisedTensor.ts?raw";
import tensorCode from "../wave-algorithms/tensor.ts?raw";

export type CodeSampleKey = AlgorithmKey | "rustNative";

export const CODE_KEYS = [
  "cx",
  "ts",
  "bin",
  "opt",
  "simd",
  "rust",
  "rustNative",
] as const satisfies readonly CodeSampleKey[];

export const CODE_NAMES: Record<CodeSampleKey, string> = {
  cx: "complex",
  ts: "tensor",
  bin: "binary",
  opt: "optimised",
  simd: "wasm simd",
  rust: "rust wasm",
  rustNative: "rust native",
};

export const CODES: Record<CodeSampleKey, string> = {
  cx: complexCode,
  ts: tensorCode,
  bin: binaryCode,
  opt: optimisedTensorCode,
  simd: simdOptimisedTensorCode,
  rust: rustOptimisedTensorCode,
  rustNative: rustNativeCode,
};
