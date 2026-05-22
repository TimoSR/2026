import type {
  BenchmarkRequest,
  BenchmarkResult,
  BenchmarkWorkerMessage,
  MetricMap,
  RankedMetric,
  TimingMap,
} from "../core/benchmarkTypes";
import { iqr, median } from "../core/stats";
import {
  FNS,
  KEYS,
  cOptCombine,
  cxCombine,
  optCombine,
  rustOptCombine,
} from "../wave-algorithms";

function emptyTimings(): TimingMap {
  const timings = {} as TimingMap;
  KEYS.forEach((key) => {
    timings[key] = [];
  });
  return timings;
}

function rankFastest(values: MetricMap): RankedMetric {
  return KEYS.reduce<RankedMetric>(
    (best, key) => (values[key] < best[1] ? [key, values[key]] : best),
    [KEYS[0], values[KEYS[0]]],
  );
}

function yieldToWorker(): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, 0);
  });
}

async function runBenchmark({
  iterations,
  trials,
  waves,
}: BenchmarkRequest): Promise<BenchmarkResult> {
  const input = new Float32Array(iterations);
  for (let index = 0; index < iterations; index += 1) {
    input[index] = index * 0.01;
  }

  const times = emptyTimings();

  for (let trial = 0; trial < trials; trial += 1) {
    postMessage({
      type: "progress",
      progress: { current: trial + 1, total: trials },
    } satisfies BenchmarkWorkerMessage);

    const order = [...KEYS].sort(() => Math.random() - 0.5);

    for (const key of order) {
      await yieldToWorker();

      const fn = FNS[key];
      const start = performance.now();

      for (let index = 0; index < iterations; index += 1) {
        fn(waves, input[index]);
      }

      times[key].push(performance.now() - start);
    }
  }

  const medians = {} as MetricMap;
  const iqrs = {} as MetricMap;
  KEYS.forEach((key) => {
    medians[key] = median(times[key]);
    iqrs[key] = iqr(times[key]);
  });

  const winner = rankFastest(medians);
  const consistent = rankFastest(iqrs);
  const margin = (((medians.cx - medians.opt) / medians.cx) * 100).toFixed(1);

  let maxError = 0;
  let cMaxError = 0;
  let rustMaxError = 0;
  for (let index = 0; index < 200; index += 1) {
    const localTime = index * 0.05;
    const optValue = optCombine(waves, localTime, 0.01);
    maxError = Math.max(
      maxError,
      Math.abs(cxCombine(waves, localTime) - optValue),
    );
    cMaxError = Math.max(
      cMaxError,
      Math.abs(optValue - cOptCombine(waves, localTime)),
    );
    rustMaxError = Math.max(
      rustMaxError,
      Math.abs(optValue - rustOptCombine(waves, localTime)),
    );
  }

  return {
    cMaxError,
    consistent,
    iqrs,
    margin,
    maxError,
    medians,
    rustMaxError,
    times,
    winner,
  };
}

self.onmessage = async (event: MessageEvent<BenchmarkRequest>) => {
  try {
    postMessage({
      type: "result",
      result: await runBenchmark(event.data),
    } satisfies BenchmarkWorkerMessage);
  } catch (error) {
    postMessage({
      type: "error",
      message: error instanceof Error ? error.message : "Benchmark failed",
    } satisfies BenchmarkWorkerMessage);
  }
};

export {};
