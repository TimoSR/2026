import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { RefObject } from "react";
import CalculationApproachPanel from "./components/CalculationApproachPanel";
import CanvasPanel from "./components/CanvasPanel";
import CodePanel from "./components/CodePanel";
import ControlButton from "./components/ControlButton";
import Explainer from "./components/Explainer";
import FairBenchPanel from "./components/FairBenchPanel";
import WaveRow from "./components/WaveRow";
import { drawCanvas, getTrackedGraphTime } from "./core/canvas";
import type { CodeSampleKey } from "./data/codeSamples";
import type { AlgorithmKey, ViewMode, Wave, WaveKey } from "./types";
import { COLORS, FNS, KEYS, NAMES } from "./wave-algorithms";
import { combineAsComplex } from "./wave-algorithms/complex";
import type { ComplexWaveResult } from "./wave-algorithms/complex";
import { combineAsTensor } from "./wave-algorithms/tensor";
import type { WaveTensorResult } from "./wave-algorithms/tensor";

const INITIAL_WAVES: Wave[] = [
  { amp: 1.0, freq: 1.0, phase: 0.0 },
  { amp: 0.5, freq: 2.0, phase: 1.0 },
  { amp: 0.3, freq: 3.0, phase: 2.5 },
];

const TIME_STEP = 0.006;
const DERIVED_UPDATE_FRAME_INTERVAL = 12;

const SUBTITLES: Record<AlgorithmKey, string> = {
  cx: "Re(sum amp * e^i theta) - baseline",
  ts: "Float32Array [cos, sin] - typed buffer",
  bin: "Q12 lookup table - bitmask - Int32Array",
  opt: "angle-step recurrence - cos-only - Float32Array",
  simd: "WebAssembly SIMD - vector cos approximation",
  rust: "Rust compiled to WebAssembly - f32 tensor",
};

const VIEW_OPTIONS: ReadonlyArray<readonly [ViewMode, string]> = [
  ["all", "All"],
  ["cx", "Complex"],
  ["ts", "Tensor"],
  ["bin", "Binary"],
  ["opt", "Optimised"],
  ["simd", "Wasm SIMD"],
  ["rust", "Rust Wasm"],
];

export default function App() {
  const [waves, setWaves] = useState<Wave[]>(INITIAL_WAVES);
  const [playing, setPlaying] = useState(true);
  const [showComponents, setShowComponents] = useState(true);
  const [view, setView] = useState<ViewMode>("all");
  const [codeMode, setCodeMode] = useState<CodeSampleKey>("cx");
  const [trackedTime, setTrackedTime] = useState(() => getTrackedGraphTime(0));
  const [complexResult, setComplexResult] = useState<ComplexWaveResult>(() =>
    combineAsComplex(INITIAL_WAVES, getTrackedGraphTime(0)),
  );
  const [tensorResult, setTensorResult] = useState<WaveTensorResult>(() =>
    combineAsTensor(INITIAL_WAVES, getTrackedGraphTime(0)),
  );

  const tRef = useRef(0);
  const frameRef = useRef(0);
  const rafRef = useRef<number | null>(null);
  const cxRef = useRef<HTMLCanvasElement | null>(null);
  const tsRef = useRef<HTMLCanvasElement | null>(null);
  const binRef = useRef<HTMLCanvasElement | null>(null);
  const optRef = useRef<HTMLCanvasElement | null>(null);
  const simdRef = useRef<HTMLCanvasElement | null>(null);
  const rustRef = useRef<HTMLCanvasElement | null>(null);

  const canvasRefs = useMemo<Record<AlgorithmKey, RefObject<HTMLCanvasElement | null>>>(
    () => ({ cx: cxRef, ts: tsRef, bin: binRef, opt: optRef, simd: simdRef, rust: rustRef }),
    [],
  );

  const animate = useCallback(() => {
    if (playing) tRef.current += TIME_STEP;

    const t = tRef.current;
    frameRef.current = (frameRef.current + 1) % DERIVED_UPDATE_FRAME_INTERVAL;

    if (playing && frameRef.current === 0) {
      const sampleTime = getTrackedGraphTime(t);
      setTrackedTime(sampleTime);
      setComplexResult(combineAsComplex(waves, sampleTime));
      setTensorResult(combineAsTensor(waves, sampleTime));
    }

    KEYS.forEach((key) => {
      if (view === "all" || view === key) {
        drawCanvas(
          canvasRefs[key].current,
          waves,
          t,
          FNS[key],
          COLORS[key],
          showComponents,
        );
      }
    });

    rafRef.current = requestAnimationFrame(animate);
  }, [canvasRefs, playing, showComponents, view, waves]);

  useEffect(() => {
    rafRef.current = requestAnimationFrame(animate);
    return () => {
      if (rafRef.current !== null) {
        cancelAnimationFrame(rafRef.current);
      }
    };
  }, [animate]);

  useEffect(() => {
    const sampleTime = getTrackedGraphTime(tRef.current);
    setTrackedTime(sampleTime);
    setComplexResult(combineAsComplex(waves, sampleTime));
    setTensorResult(combineAsTensor(waves, sampleTime));
  }, [waves]);

  const updateWave = (index: number, key: WaveKey, value: number) => {
    setWaves((current) =>
      current.map((wave, waveIndex) =>
        waveIndex === index ? { ...wave, [key]: value } : wave,
      ),
    );
  };

  const removeWave = (index: number) => {
    setWaves((current) => current.filter((_, waveIndex) => waveIndex !== index));
  };

  const addWave = () => {
    setWaves((current) => {
      if (current.length >= 6) return current;

      return [
        ...current,
        {
          amp: 0.4,
          freq: Number((Math.random() * 3 + 1).toFixed(1)),
          phase: Number((Math.random() * 6).toFixed(1)),
        },
      ];
    });
  };

  const updateView = (nextView: ViewMode) => {
    setView(nextView);

    if (nextView !== "all") {
      setCodeMode(nextView);
    }
  };

  return (
    <main className="app">
      <section className="app-shell">
        <header className="app-header">
          <div>
            <h1>Wave Simulation</h1>
            <p>Complex, tensor, binary, optimised tensor, WebAssembly SIMD, and Rust wasm representations.</p>
          </div>
        </header>

        <div className="workspace-grid">
          <aside className="components-sidebar" aria-label="Wave components">
            <section className="panel wave-components-panel">
              <div className="panel-heading-row">
                <div className="panel-heading">Wave components</div>
                <div className="panel-actions">
                  <ControlButton
                    active={showComponents}
                    onClick={() => setShowComponents((current) => !current)}
                  >
                    Components
                  </ControlButton>
                  <ControlButton onClick={addWave}>+ Wave</ControlButton>
                </div>
              </div>
              <div className="wave-list">
                {waves.map((wave, index) => (
                  <WaveRow
                    key={index}
                    wave={wave}
                    index={index}
                    onChange={updateWave}
                    onRemove={removeWave}
                  />
                ))}
              </div>
              <p className="panel-note">
                These inputs feed every renderer in the centered simulation view.
              </p>
            </section>
          </aside>

          <div className="simulation-column">
            <div className="toolbar" aria-label="Simulation controls">
              <div className="segmented-control" aria-label="Wave view">
                {VIEW_OPTIONS.map(([value, label]) => (
                  <ControlButton
                    key={value}
                    active={view === value}
                    onClick={() => updateView(value)}
                  >
                    {label}
                  </ControlButton>
                ))}
              </div>

              <div className="toolbar-actions" />
            </div>

            <section className="canvas-stack" aria-label="Wave renderers">
              {KEYS.map(
                (key) =>
                  (view === "all" || view === key) && (
                    <CanvasPanel
                      key={key}
                      label={NAMES[key].toUpperCase()}
                      subtitle={SUBTITLES[key]}
                      color={COLORS[key]}
                      canvasRef={canvasRefs[key]}
                      selected={codeMode === key}
                      onSelect={() => setCodeMode(key)}
                    />
                  ),
              )}
            </section>

            <CalculationApproachPanel
              complexResult={complexResult}
              onPlayingChange={() => setPlaying((current) => !current)}
              playing={playing}
              sampleTime={trackedTime}
              tensorResult={tensorResult}
            />

            <FairBenchPanel waves={waves} />

            <Explainer />
          </div>

          <aside className="code-sidebar" aria-label="Wave algorithm source">
            <CodePanel codeMode={codeMode} onCodeModeChange={setCodeMode} />
          </aside>
        </div>
      </section>
    </main>
  );
}
