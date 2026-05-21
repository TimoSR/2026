import { useCallback, useState } from "react";
import { iqr, median } from "../core/stats";
import type { AlgorithmKey, Wave } from "../types";
import {
  COLORS,
  FNS,
  KEYS,
  NAMES,
  cxCombine,
  isRustOptReady,
  isSimdOptReady,
  optCombine,
  rustOptCombine,
  simdOptCombine,
} from "../wave-algorithms";
import DotChart from "./DotChart";

const TRIAL_OPTIONS = [5, 11, 21];
const ITERATION_OPTIONS: ReadonlyArray<readonly [number, string]> = [
  [10000, "10k"],
  [50000, "50k"],
  [200000, "200k"],
];

type Progress = {
  current: number;
  total: number;
};

type TimingMap = Record<AlgorithmKey, number[]>;
type MetricMap = Record<AlgorithmKey, number>;
type RankedMetric = readonly [AlgorithmKey, number];

type BenchmarkResult = {
  consistent: RankedMetric;
  iqrs: MetricMap;
  margin: string;
  maxError: number;
  medians: MetricMap;
  rustMaxError: number;
  simdMaxError: number;
  times: TimingMap;
  winner: RankedMetric;
};

type FairBenchPanelProps = {
  waves: readonly Wave[];
};

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

export default function FairBenchPanel({ waves }: FairBenchPanelProps) {
  const [trials, setTrials] = useState(11);
  const [iterations, setIterations] = useState(50000);
  const [running, setRunning] = useState(false);
  const [progress, setProgress] = useState<Progress | null>(null);
  const [result, setResult] = useState<BenchmarkResult | null>(null);

  const run = useCallback(async () => {
    setRunning(true);
    setResult(null);

    const input = new Float32Array(iterations);
    for (let index = 0; index < iterations; index += 1) {
      input[index] = index * 0.01;
    }

    const times = emptyTimings();

    for (let trial = 0; trial < trials; trial += 1) {
      setProgress({ current: trial + 1, total: trials });
      const order = [...KEYS].sort(() => Math.random() - 0.5);

      await new Promise((resolve) => {
        setTimeout(resolve, 0);
      });

      for (const key of order) {
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
    let simdMaxError = 0;
    let rustMaxError = 0;
    for (let index = 0; index < 200; index += 1) {
      const localTime = index * 0.05;
      const optValue = optCombine(waves, localTime, 0.01);
      maxError = Math.max(
        maxError,
        Math.abs(cxCombine(waves, localTime) - optValue),
      );
      simdMaxError = Math.max(
        simdMaxError,
        Math.abs(optValue - simdOptCombine(waves, localTime)),
      );
      rustMaxError = Math.max(
        rustMaxError,
        Math.abs(optValue - rustOptCombine(waves, localTime)),
      );
    }

    setProgress(null);
    setResult({
      consistent,
      iqrs,
      margin,
      maxError,
      medians,
      rustMaxError,
      simdMaxError,
      times,
      winner,
    });
    setRunning(false);
  }, [iterations, trials, waves]);

  const maxMedian = result ? Math.max(...Object.values(result.medians)) : 1;

  return (
    <section className="panel bench-panel">
      <header className="bench-header">
        <div>
          <h2>Fair benchmark</h2>
          <p>Randomised order, median of N trials.</p>
        </div>

        <div className="bench-controls">
          <label>
            <span>Trials</span>
            <select
              value={trials}
              onChange={(event) => setTrials(Number.parseInt(event.target.value, 10))}
            >
              {TRIAL_OPTIONS.map((value) => (
                <option key={value} value={value}>
                  {value}
                </option>
              ))}
            </select>
          </label>

          <label>
            <span>Iters</span>
            <select
              value={iterations}
              onChange={(event) => setIterations(Number.parseInt(event.target.value, 10))}
            >
              {ITERATION_OPTIONS.map(([value, label]) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </label>

          <button
            type="button"
            className="control-button"
            onClick={run}
            disabled={running}
          >
            {running ? "Running" : "Run"}
          </button>
        </div>
      </header>

      {progress && (
        <div className="progress-block">
          <div className="progress-label">
            Running trial {progress.current} of {progress.total}
          </div>
          <div className="progress-track">
            <span
              className="progress-fill"
              style={{ width: `${((progress.current / progress.total) * 100).toFixed(0)}%` }}
            />
          </div>
        </div>
      )}

      <div className="benchmark-grid">
        {KEYS.map((key) => {
          const med = result?.medians[key];
          const ops = med ? Math.round((iterations / med) * 1000) : null;
          const min = result ? Math.min(...result.times[key]) : null;
          const max = result ? Math.max(...result.times[key]) : null;

          return (
            <article key={key} className="benchmark-card">
              <div className="metric-row">
                <span style={{ color: COLORS[key] }}>{NAMES[key]}</span>
                <span>{med != null ? `${med.toFixed(2)}ms` : "-"}</span>
              </div>
              <div className="metric-bar">
                <span
                  style={{
                    width: med != null ? `${((med / maxMedian) * 100).toFixed(1)}%` : "0%",
                    background: COLORS[key],
                  }}
                />
              </div>
              <div className="metric-subtext">
                {ops != null ? `${ops.toLocaleString()} ops/s` : "run to see"}
              </div>
              {result && (
                <div className="metric-subtext">
                  {min?.toFixed(1)}-{max?.toFixed(1)}ms
                </div>
              )}
              <DotChart times={result?.times[key]} color={COLORS[key]} />
            </article>
          );
        })}
      </div>

      <p className="bench-note">
        Each dot is one trial. Darker dots are slower. Wasm SIMD{" "}
        {isSimdOptReady() ? "is active" : "fell back to optimised tensor"}. Rust wasm{" "}
        {isRustOptReady() ? "is active" : "fell back to optimised tensor"}.
      </p>

      {result && (
        <>
          <div className="summary-grid">
            {[
              ["winner", NAMES[result.winner[0]]],
              ["opt vs complex", `${Number.parseFloat(result.margin) > 0 ? "+" : ""}${result.margin}%`],
              ["most consistent", NAMES[result.consistent[0]]],
              ["opt max error", result.maxError.toFixed(4)],
              ["simd max error", result.simdMaxError.toFixed(4)],
              ["rust max error", result.rustMaxError.toFixed(4)],
            ].map(([label, value]) => (
              <div key={label} className="summary-tile">
                <strong>{value}</strong>
                <span>{label}</span>
              </div>
            ))}
          </div>

          <p className="panel-note">
            Median of {trials} trials, {iterations.toLocaleString()} iterations each,
            randomised order. Optimised is {result.margin}%{" "}
            {Number.parseFloat(result.margin) > 0 ? "faster" : "slower"} than complex.
            Any margin under about 5% is usually runtime noise.
          </p>
        </>
      )}
    </section>
  );
}
