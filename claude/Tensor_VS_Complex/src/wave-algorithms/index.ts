import type { AlgorithmKey, WaveCombiner } from "../types";
import { binCombine } from "./binary";
import { cxCombine } from "./complex";
import { optCombine } from "./optimisedTensor";
import { isRustOptReady, rustOptCombine } from "./rustOptimisedTensor";
import { tsCombine } from "./tensor";

export {
  binCombine,
  cxCombine,
  isRustOptReady,
  optCombine,
  rustOptCombine,
  tsCombine,
};

export const FNS: Record<AlgorithmKey, WaveCombiner> = {
  cx: cxCombine,
  ts: tsCombine,
  bin: binCombine,
  opt: (waves, t) => optCombine(waves, t, 0.01),
  rust: rustOptCombine,
};

export const COLORS: Record<AlgorithmKey, string> = {
  cx: "#4ea5f5",
  ts: "#ad7eea",
  bin: "#45c58b",
  opt: "#f5a524",
  rust: "#38d5c8",
};

export const NAMES: Record<AlgorithmKey, string> = {
  cx: "complex",
  ts: "tensor",
  bin: "binary",
  opt: "optimised",
  rust: "rust wasm",
};

export const KEYS = ["cx", "ts", "bin", "opt", "rust"] as const satisfies readonly AlgorithmKey[];
