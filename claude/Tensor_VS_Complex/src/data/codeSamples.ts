import type { AlgorithmKey } from "../types";
import binaryCode from "../wave-algorithms/binary.ts?raw";
import complexCode from "../wave-algorithms/complex.ts?raw";
import optimisedTensorCode from "../wave-algorithms/optimisedTensor.ts?raw";
import rustOptimisedTensorCode from "../../rust/optimized-tensor/src/lib.rs?raw";
import tensorCode from "../wave-algorithms/tensor.ts?raw";

export type CodeSampleKey = AlgorithmKey;

export const CODE_KEYS = [
  "cx",
  "ts",
  "bin",
  "opt",
  "rust",
] as const satisfies readonly CodeSampleKey[];

export const CODE_NAMES: Record<CodeSampleKey, string> = {
  cx: "complex",
  ts: "tensor",
  bin: "binary",
  opt: "optimised",
  rust: "rust wasm",
};

export const CODES: Record<CodeSampleKey, string> = {
  cx: complexCode,
  ts: tensorCode,
  bin: binaryCode,
  opt: optimisedTensorCode,
  rust: rustOptimisedTensorCode,
};
