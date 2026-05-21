import type { AlgorithmKey, WaveCombiner } from "../types";
import { binCombine } from "./binary";
import { cxCombine } from "./complex";
import { optCombine } from "./optimisedTensor";
import { tsCombine } from "./tensor";

export { binCombine, cxCombine, optCombine, tsCombine };

export const FNS: Record<AlgorithmKey, WaveCombiner> = {
  cx: cxCombine,
  ts: tsCombine,
  bin: binCombine,
  opt: (waves, t) => optCombine(waves, t, 0.01),
};

export const COLORS: Record<AlgorithmKey, string> = {
  cx: "#4ea5f5",
  ts: "#ad7eea",
  bin: "#45c58b",
  opt: "#f5a524",
};

export const NAMES: Record<AlgorithmKey, string> = {
  cx: "complex",
  ts: "tensor",
  bin: "binary",
  opt: "optimised",
};

export const KEYS = ["cx", "ts", "bin", "opt"] as const satisfies readonly AlgorithmKey[];
