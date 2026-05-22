export type Wave = {
  amp: number;
  freq: number;
  phase: number;
};

export type WaveKey = keyof Wave;
export type AlgorithmKey = "cx" | "ts" | "bin" | "opt" | "c" | "rust";
export type ViewMode = "all" | AlgorithmKey;
export type WaveCombiner = (waves: readonly Wave[], t: number) => number;
