import type { AlgorithmKey } from "../types";
import binaryCode from "../wave-algorithms/binary.ts?raw";
import complexCode from "../wave-algorithms/complex.ts?raw";
import optimisedTensorCode from "../wave-algorithms/optimisedTensor.ts?raw";
import rustOptimisedTensorCode from "../../rust/optimized-tensor/src/lib.rs?raw";
import simdOptimisedTensorCode from "../wave-algorithms/simdOptimisedTensor.ts?raw";
import tensorCode from "../wave-algorithms/tensor.ts?raw";

export const CODES: Record<AlgorithmKey, string> = {
  cx: complexCode,
  ts: tensorCode,
  bin: binaryCode,
  opt: optimisedTensorCode,
  simd: simdOptimisedTensorCode,
  rust: rustOptimisedTensorCode,
};
