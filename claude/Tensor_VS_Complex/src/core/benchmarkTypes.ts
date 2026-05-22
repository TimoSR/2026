import type { AlgorithmKey, Wave } from "../types";

export type BenchmarkProgress = {
  current: number;
  total: number;
};

export type TimingMap = Record<AlgorithmKey, number[]>;
export type MetricMap = Record<AlgorithmKey, number>;
export type RankedMetric = readonly [AlgorithmKey, number];

export type BenchmarkResult = {
  consistent: RankedMetric;
  iqrs: MetricMap;
  margin: string;
  maxError: number;
  medians: MetricMap;
  rustMaxError: number;
  times: TimingMap;
  winner: RankedMetric;
};

export type BenchmarkRequest = {
  iterations: number;
  trials: number;
  waves: readonly Wave[];
};

export type BenchmarkWorkerMessage =
  | { type: "progress"; progress: BenchmarkProgress }
  | { type: "result"; result: BenchmarkResult }
  | { type: "error"; message: string };
