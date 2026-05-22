import { useCallback, useEffect, useRef, useState } from "react";
import type {
  BenchmarkProgress,
  BenchmarkRequest,
  BenchmarkResult,
  BenchmarkWorkerMessage,
} from "../core/benchmarkTypes";
import type { Wave } from "../types";
import {
  COLORS,
  KEYS,
  NAMES,
  isCOptReady,
  isRustOptReady,
} from "../wave-algorithms";
import DotChart from "./DotChart";

const TRIAL_OPTIONS = [5, 11, 21];
const ITERATION_OPTIONS: ReadonlyArray<readonly [number, string]> = [
  [10000, "10k"],
  [50000, "50k"],
  [200000, "200k"],
];

type FairBenchPanelProps = {
  waves: readonly Wave[];
};

export default function FairBenchPanel({ waves }: FairBenchPanelProps) {
  const [trials, setTrials] = useState(11);
  const [iterations, setIterations] = useState(50000);
  const [running, setRunning] = useState(false);
  const [progress, setProgress] = useState<BenchmarkProgress | null>(null);
  const [result, setResult] = useState<BenchmarkResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const workerRef = useRef<Worker | null>(null);

  const stopBenchmark = useCallback(() => {
    workerRef.current?.terminate();
    workerRef.current = null;
    setProgress(null);
    setRunning(false);
  }, []);

  useEffect(() => stopBenchmark, [stopBenchmark]);

  const run = useCallback(() => {
    workerRef.current?.terminate();

    setRunning(true);
    setError(null);
    setResult(null);
    setProgress({ current: 0, total: trials });

    const worker = new Worker(
      new URL("../workers/benchmarkWorker.ts", import.meta.url),
      { type: "module" },
    );

    workerRef.current = worker;

    worker.onmessage = (event: MessageEvent<BenchmarkWorkerMessage>) => {
      if (workerRef.current !== worker) return;

      if (event.data.type === "progress") {
        setProgress(event.data.progress);
        return;
      }

      if (event.data.type === "result") {
        setResult(event.data.result);
        setProgress(null);
        setRunning(false);
        worker.terminate();
        workerRef.current = null;
        return;
      }

      setError(event.data.message);
      setProgress(null);
      setRunning(false);
      worker.terminate();
      workerRef.current = null;
    };

    worker.onerror = (event) => {
      if (workerRef.current !== worker) return;

      setError(event.message || "Benchmark worker failed");
      setProgress(null);
      setRunning(false);
      worker.terminate();
      workerRef.current = null;
    };

    const request: BenchmarkRequest = {
      iterations,
      trials,
      waves: waves.map(({ amp, freq, phase }) => ({ amp, freq, phase })),
    };

    worker.postMessage(request);
  }, [iterations, trials, waves]);

  const maxMedian = result ? Math.max(...Object.values(result.medians)) : 1;

  return (
    <section className="panel bench-panel">
      <header className="bench-header">
        <div>
          <h2>Benchmark</h2>
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
            onClick={running ? stopBenchmark : run}
          >
            {running ? "Cancel" : "Run"}
          </button>
        </div>
      </header>

      {progress && (
        <div className="progress-block">
          <div className="progress-label">
            {progress.current === 0
              ? "Preparing benchmark"
              : `Running trial ${progress.current} of ${progress.total}`}
          </div>
          <div className="progress-track">
            <span
              className="progress-fill"
              style={{ width: `${((progress.current / progress.total) * 100).toFixed(0)}%` }}
            />
          </div>
        </div>
      )}

      {error && <div className="benchmark-error">{error}</div>}

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
        Benchmarks run in a background worker. Each dot is one trial. Darker dots are slower.
        C wasm {isCOptReady() ? "is active" : "fell back to optimised tensor"}.
        Rust wasm {isRustOptReady() ? "is active" : "fell back to optimised tensor"}.
      </p>

      {result && (
        <>
          <div className="summary-grid">
            {[
              ["winner", NAMES[result.winner[0]]],
              ["opt vs complex", `${Number.parseFloat(result.margin) > 0 ? "+" : ""}${result.margin}%`],
              ["most consistent", NAMES[result.consistent[0]]],
              ["opt max error", result.maxError.toFixed(4)],
              ["c max error", result.cMaxError.toFixed(4)],
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
