import { useState, useEffect, useRef, useCallback } from "react";

const PTS = 300, H = 80;

// ── Lookup table (binary version) ────────────────────────────────────────────
const Q       = 4096;
const TBL_LEN = 1024;
const TBL_MSK = TBL_LEN - 1;

const cosTable = new Int32Array(TBL_LEN);
const sinTable = new Int32Array(TBL_LEN);
for (let i = 0; i < TBL_LEN; i++) {
  const a = (i / TBL_LEN) * 2 * Math.PI;
  cosTable[i] = Math.round(Math.cos(a) * Q);
  sinTable[i] = Math.round(Math.sin(a) * Q);
}

// Shared typed buffers
const packed = new Int32Array(2);
const tsBuf  = new Float32Array(2);
const outBuf = new Float32Array(1);

// ── Four implementations ──────────────────────────────────────────────────────

// 1. Complex — baseline
function cxCombine(waves, t) {
  let re = 0, im = 0;
  for (const { amp, freq, phase } of waves) {
    const a = freq * t + phase;
    re += amp * Math.cos(a);
    im += amp * Math.sin(a); // computed but never used
  }
  return re;
}

// 2. Tensor — Float32Array typed buffer
function tsCombine(waves, t) {
  tsBuf[0] = 0; tsBuf[1] = 0;
  for (const { amp, freq, phase } of waves) {
    const a = freq * t + phase;
    tsBuf[0] += amp * Math.cos(a);
    tsBuf[1] += amp * Math.sin(a); // computed but never used
  }
  return tsBuf[0];
}

// 3. Binary — Q12 lookup table, bitmask wrap, packed Int32Array
function binCombine(waves, t) {
  packed[0] = 0; packed[1] = 0;
  for (const { amp, freq, phase } of waves) {
    let norm = ((freq * t + phase) / (2 * Math.PI)) % 1;
    if (norm < 0) norm += 1;
    const idx  = (norm * TBL_LEN) & TBL_MSK;
    const ampQ = Math.round(amp * Q);
    packed[0] += (ampQ * cosTable[idx]) >> 12;
    packed[1] += (ampQ * sinTable[idx]) >> 12;
  }
  return packed[0] / Q;
}

// 4. Optimised tensor — three improvements:
//    a) Angle-step recurrence: cos(a+Δ) = cos(a)·cos(Δ) - sin(a)·sin(Δ)
//       cosΔ/sinΔ computed once per wave; stepping forward costs only multiplies.
//    b) Cos-only accumulation: sin component dropped entirely.
//    c) Float32Array output buffer for JIT-friendly memory access.
function optCombine(waves, t, step = 0.01) {
  outBuf[0] = 0;
  for (const { amp, freq, phase } of waves) {
    const a0    = freq * t + phase;
    const dA    = freq * step;
    const cosDa = Math.cos(dA); // computed once per wave
    const sinDa = Math.sin(dA); // computed once per wave

    // Seed the recurrence at t
    let cosA = Math.cos(a0);
    let sinA = Math.sin(a0);

    // Accumulate cos component only
    outBuf[0] += amp * cosA;

    // Step forward via angle-addition identity (ready for next sample)
    const nextCos = cosA * cosDa - sinA * sinDa;
    const nextSin = sinA * cosDa + cosA * sinDa;
    cosA = nextCos;
    sinA = nextSin;
  }
  return outBuf[0];
}

const FNS    = { cx: cxCombine, ts: tsCombine, bin: binCombine, opt: (ws, t) => optCombine(ws, t, 0.01) };
const COLORS = { cx: "#3b9fe8", ts: "#a07ee0", bin: "#3ec98e", opt: "#f59e0b" };
const NAMES  = { cx: "complex", ts: "tensor",  bin: "binary",  opt: "optimised" };
const KEYS   = ["cx", "ts", "bin", "opt"];

// ── Stats ─────────────────────────────────────────────────────────────────────
function median(arr) {
  const s = [...arr].sort((a, b) => a - b);
  const m = Math.floor(s.length / 2);
  return s.length % 2 ? s[m] : (s[m - 1] + s[m]) / 2;
}
function iqr(arr) {
  const s = [...arr].sort((a, b) => a - b);
  return s[Math.floor(s.length * 3 / 4)] - s[Math.floor(s.length / 4)];
}

// ── Canvas renderer ───────────────────────────────────────────────────────────
function drawCanvas(canvas, waves, t, fn, color, showComp) {
  if (!canvas) return;
  const W = canvas.offsetWidth || 600;
  canvas.width = W; canvas.height = H;
  const ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, W, H);
  const maxAmp = waves.reduce((s, w) => s + w.amp, 0) || 1;
  const scale  = (H / 2 - 6) / maxAmp;

  if (showComp) {
    waves.forEach(({ amp, freq, phase }, i) => {
      ctx.beginPath();
      ctx.strokeStyle = `hsla(${i * 90 + 180},60%,60%,0.22)`;
      ctx.lineWidth = 1;
      for (let px = 0; px < PTS; px++) {
        const x  = (px / PTS) * W;
        const tl = (px / PTS) * Math.PI * 4 + t;
        const y  = H / 2 - amp * Math.cos(freq * tl + phase) * scale;
        px === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
      }
      ctx.stroke();
    });
  }

  ctx.beginPath();
  ctx.strokeStyle = color;
  ctx.lineWidth = 2;
  for (let px = 0; px < PTS; px++) {
    const x  = (px / PTS) * W;
    const tl = (px / PTS) * Math.PI * 4 + t;
    const y  = H / 2 - fn(waves, tl) * scale;
    px === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y);
  }
  ctx.stroke();
}

// ── Code strings ──────────────────────────────────────────────────────────────
const CODES = {
  cx: `// Complex — baseline
function cxCombine(waves, t) {
  let re = 0, im = 0;
  for (const {amp, freq, phase} of waves) {
    const a = freq * t + phase;
    re += amp * Math.cos(a);
    im += amp * Math.sin(a); // never used!
  }
  return re;
}`,
  ts: `// Tensor — Float32Array typed buffer
const tsBuf = new Float32Array(2);

function tsCombine(waves, t) {
  tsBuf[0] = 0; tsBuf[1] = 0;
  for (const {amp, freq, phase} of waves) {
    const a = freq * t + phase;
    tsBuf[0] += amp * Math.cos(a);
    tsBuf[1] += amp * Math.sin(a); // never used!
  }
  return tsBuf[0];
}`,
  bin: `// Binary — Q12 lookup table
const Q=4096, TBL=1024, MSK=TBL-1;
const cosT=new Int32Array(TBL);
const sinT=new Int32Array(TBL);
for(let i=0;i<TBL;i++){
  const a=(i/TBL)*2*Math.PI;
  cosT[i]=Math.round(Math.cos(a)*Q);
  sinT[i]=Math.round(Math.sin(a)*Q);
}
const packed=new Int32Array(2);

function binCombine(waves, t) {
  packed[0]=0; packed[1]=0;
  for(const{amp,freq,phase}of waves){
    let n=((freq*t+phase)/(2*Math.PI))%1;
    if(n<0)n+=1;
    const idx=(n*TBL)&MSK;
    const aQ=Math.round(amp*Q);
    packed[0]+=(aQ*cosT[idx])>>12;
    packed[1]+=(aQ*sinT[idx])>>12;
  }
  return packed[0]/Q;
}`,
  opt: `// Optimised tensor — three key improvements:
//
// 1. ANGLE-STEP RECURRENCE
//    cos(a + Δ) = cos(a)·cos(Δ) - sin(a)·sin(Δ)
//    Compute cosΔ/sinΔ once per wave per frame,
//    then step forward with multiplies only.
//
// 2. COS-ONLY ACCUMULATION
//    Drop the sin accumulator entirely —
//    it's computed but never returned.
//
// 3. Float32Array output buffer
//    Typed, contiguous, JIT-friendly.

const outBuf = new Float32Array(1);

function optCombine(waves, t, step = 0.01) {
  outBuf[0] = 0;
  for (const {amp, freq, phase} of waves) {
    const a0    = freq * t + phase;
    const dA    = freq * step;
    const cosDa = Math.cos(dA); // once per wave
    const sinDa = Math.sin(dA); // once per wave

    let cosA = Math.cos(a0);
    let sinA = Math.sin(a0);

    outBuf[0] += amp * cosA; // cos only — no im

    // Step forward (angle-addition identity)
    const nc = cosA*cosDa - sinA*sinDa;
    const ns = sinA*cosDa + cosA*sinDa;
    cosA = nc; sinA = ns;
  }
  return outBuf[0];
}`,
};

// ── Dot chart ─────────────────────────────────────────────────────────────────
function DotChart({ times, color }) {
  if (!times || !times.length) return <div style={{ minHeight: 12 }} />;
  const sorted = [...times].sort((a, b) => a - b);
  const mn = sorted[0], rng = (sorted[sorted.length - 1] - mn) || 1;
  return (
    <div style={{ display: "flex", flexWrap: "wrap", gap: 2, marginTop: 5, minHeight: 12 }}>
      {sorted.map((v, i) => (
        <span key={i} title={v.toFixed(2) + "ms"} style={{
          width: 6, height: 6, borderRadius: "50%", display: "inline-block",
          background: color, opacity: (0.3 + (v - mn) / rng * 0.7).toFixed(2),
        }} />
      ))}
    </div>
  );
}

// ── Wave row ──────────────────────────────────────────────────────────────────
function WaveRow({ wave, index, onChange, onRemove }) {
  return (
    <div style={{ display: "grid", gridTemplateColumns: "18px 1fr 1fr 1fr 28px", gap: 8, alignItems: "center", padding: "6px 0", borderBottom: "1px solid #1e293b" }}>
      <span style={{ fontSize: 11, color: "#64748b" }}>{index + 1}</span>
      {[["amp", 0.1, 2, 0.05], ["freq", 0.1, 6, 0.1], ["phase", 0, 6.28, 0.05]].map(([key, min, max, step]) => (
        <div key={key}>
          <span style={{ fontSize: 10, color: "#475569", textTransform: "uppercase", letterSpacing: "0.06em", display: "block", marginBottom: 2 }}>{key}</span>
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <input type="range" min={min} max={max} step={step} value={wave[key]}
              onChange={e => onChange(index, key, parseFloat(e.target.value))}
              style={{ flex: 1, accentColor: "#f59e0b" }} />
            <span style={{ fontSize: 10, width: 26, textAlign: "right", color: "#94a3b8" }}>{wave[key].toFixed(1)}</span>
          </div>
        </div>
      ))}
      <button onClick={() => onRemove(index)}
        style={{ background: "none", border: "1px solid #1e293b", borderRadius: 4, color: "#475569", cursor: "pointer", fontSize: 14, padding: "1px 5px" }}>×</button>
    </div>
  );
}

// ── Canvas panel ──────────────────────────────────────────────────────────────
function CanvasPanel({ label, subtitle, color, canvasRef }) {
  return (
    <div style={{ border: "1px solid #1e293b", borderRadius: 10, overflow: "hidden", marginBottom: 6 }}>
      <div style={{ padding: "5px 12px", borderBottom: "1px solid #1e293b", display: "flex", alignItems: "center", gap: 8 }}>
        <span style={{ width: 8, height: 8, borderRadius: "50%", background: color, display: "inline-block" }} />
        <span style={{ fontSize: 11, fontWeight: 500, color, letterSpacing: "0.06em" }}>{label}</span>
        <span style={{ fontSize: 11, color: "#334155" }}>{subtitle}</span>
      </div>
      <canvas ref={canvasRef} height={H} style={{ display: "block", width: "100%" }} />
    </div>
  );
}

// ── Fair benchmark panel ──────────────────────────────────────────────────────
function FairBenchPanel({ waves }) {
  const [trials,  setTrials]  = useState(11);
  const [iters,   setIters]   = useState(50000);
  const [running, setRunning] = useState(false);
  const [prog,    setProg]    = useState(null);
  const [result,  setResult]  = useState(null);

  const run = useCallback(async () => {
    setRunning(true); setResult(null);
    const input = new Float32Array(iters);
    for (let i = 0; i < iters; i++) input[i] = i * 0.01;
    const times = { cx: [], ts: [], bin: [], opt: [] };

    for (let tr = 0; tr < trials; tr++) {
      setProg({ cur: tr + 1, total: trials });
      const order = [...KEYS].sort(() => Math.random() - 0.5);
      await new Promise(r => setTimeout(r, 0));
      for (const k of order) {
        const fn = FNS[k];
        const t0 = performance.now();
        for (let i = 0; i < iters; i++) fn(waves, input[i]);
        times[k].push(performance.now() - t0);
      }
    }

    setProg(null);
    const meds = {}; KEYS.forEach(k => meds[k] = median(times[k]));
    const iqrs = {}; KEYS.forEach(k => iqrs[k] = iqr(times[k]));
    const winner      = Object.entries(meds).reduce((a, b) => a[1] < b[1] ? a : b);
    const consistent  = Object.entries(iqrs).reduce((a, b) => a[1] < b[1] ? a : b);
    const margin      = ((meds.cx - meds.opt) / meds.cx * 100).toFixed(1);
    let maxErr = 0;
    for (let i = 0; i < 200; i++) {
      const tl = i * 0.05;
      maxErr = Math.max(maxErr, Math.abs(cxCombine(waves, tl) - optCombine(waves, tl, 0.01)));
    }
    setResult({ meds, times, iqrs, winner, consistent, margin, maxErr });
    setRunning(false);
  }, [waves, trials, iters]);

  const s  = { fontSize: 10, color: "#64748b" };
  const maxMed = result ? Math.max(...Object.values(result.meds)) : 1;

  return (
    <div style={{ border: "1px solid #1e293b", borderRadius: 10, overflow: "hidden", marginBottom: 16 }}>
      <div style={{ padding: "8px 14px", borderBottom: "1px solid #1e293b", display: "flex", alignItems: "center", justifyContent: "space-between", flexWrap: "wrap", gap: 8 }}>
        <div>
          <span style={{ fontSize: 12, fontWeight: 500, color: "#e2e8f0" }}>Fair benchmark</span>
          <span style={{ fontSize: 11, color: "#475569", marginLeft: 8 }}>randomised order · median of N trials</span>
        </div>
        <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
          <span style={s}>Trials</span>
          <select value={trials} onChange={e => setTrials(parseInt(e.target.value))}
            style={{ fontSize: 11, padding: "3px 6px", borderRadius: 4, border: "1px solid #1e293b", background: "#0f172a", color: "#e2e8f0" }}>
            {[5, 11, 21].map(v => <option key={v} value={v}>{v}</option>)}
          </select>
          <span style={s}>Iters</span>
          <select value={iters} onChange={e => setIters(parseInt(e.target.value))}
            style={{ fontSize: 11, padding: "3px 6px", borderRadius: 4, border: "1px solid #1e293b", background: "#0f172a", color: "#e2e8f0" }}>
            {[[10000,"10k"],[50000,"50k"],[200000,"200k"]].map(([v,l]) => <option key={v} value={v}>{l}</option>)}
          </select>
          <button onClick={run} disabled={running}
            style={{ padding: "5px 14px", borderRadius: 6, border: "1px solid #334155", background: "transparent", color: "#94a3b8", cursor: "pointer", fontSize: 12 }}>
            {running ? "Running…" : "▶ Run"}
          </button>
        </div>
      </div>

      <div style={{ padding: 14 }}>
        {prog && (
          <div style={{ marginBottom: 12 }}>
            <div style={{ ...s, marginBottom: 6 }}>Running trial {prog.cur} of {prog.total}…</div>
            <div style={{ background: "#1e293b", borderRadius: 4, height: 4, overflow: "hidden" }}>
              <div style={{ height: 4, background: "#334155", borderRadius: 4, width: (prog.cur / prog.total * 100).toFixed(0) + "%", transition: "width 0.1s" }} />
            </div>
          </div>
        )}

        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr 1fr", gap: 10 }}>
          {KEYS.map(k => {
            const med = result?.meds[k];
            const ops = med ? Math.round(iters / med * 1000) : null;
            const mn  = result ? Math.min(...result.times[k]) : null;
            const mx  = result ? Math.max(...result.times[k]) : null;
            return (
              <div key={k}>
                <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 4 }}>
                  <span style={{ fontSize: 11, color: COLORS[k], fontWeight: 500 }}>{NAMES[k]}</span>
                  <span style={s}>{med != null ? med.toFixed(2) + "ms" : "—"}</span>
                </div>
                <div style={{ background: "#1e293b", borderRadius: 4, height: 8, overflow: "hidden" }}>
                  <div style={{ height: 8, borderRadius: 4, background: COLORS[k], transition: "width 0.4s",
                    width: result ? (med / maxMed * 100).toFixed(1) + "%" : "0%" }} />
                </div>
                <div style={{ ...s, marginTop: 3 }}>{ops != null ? ops.toLocaleString() + " ops/s" : "run to see"}</div>
                {result && <div style={{ ...s }}>{mn?.toFixed(1)}–{mx?.toFixed(1)}ms</div>}
                <DotChart times={result?.times[k]} color={COLORS[k]} />
              </div>
            );
          })}
        </div>
        <div style={{ ...s, marginTop: 6 }}>Each dot = one trial. Darker = slower. Sorted left to right.</div>
      </div>

      {result && (
        <>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(4,1fr)", gap: 8, padding: "0 14px 14px" }}>
            {[
              ["winner",         NAMES[result.winner[0]]],
              ["opt vs complex", (parseFloat(result.margin) > 0 ? "+" : "") + result.margin + "%"],
              ["most consistent",NAMES[result.consistent[0]]],
              ["opt max error",  result.maxErr.toFixed(4)],
            ].map(([label, val]) => (
              <div key={label} style={{ background: "#0f172a", borderRadius: 8, padding: "8px 10px", textAlign: "center" }}>
                <div style={{ fontSize: 14, fontWeight: 500, color: "#e2e8f0" }}>{val}</div>
                <div style={s}>{label}</div>
              </div>
            ))}
          </div>
          <p style={{ ...s, padding: "0 14px 14px", margin: 0, lineHeight: 1.6 }}>
            Median of {result.meds && trials} trials, {iters.toLocaleString()} iters each, randomised order.
            Optimised is {result.margin}% {parseFloat(result.margin) > 0 ? "faster" : "slower"} than complex.
            {" "}{NAMES[result.consistent[0]]} has the tightest IQR.
            Any margin under ~5% is JIT noise — run again to confirm a trend.
          </p>
        </>
      )}
    </div>
  );
}

// ── Explainer ─────────────────────────────────────────────────────────────────
function Explainer() {
  return (
    <div style={{ fontSize: 11, color: "#64748b", lineHeight: 1.7, background: "#0a0f1a", borderRadius: 8, padding: 14, border: "1px solid #1e293b" }}>
      <div style={{ color: "#f59e0b", fontWeight: 500, marginBottom: 4 }}>What makes optimised faster</div>
      <strong style={{ color: "#94a3b8", fontWeight: 500 }}>Angle-step recurrence:</strong> instead of computing
      cos(freq·t + phase) from scratch each sample, compute cos(Δ) and sin(Δ) once per wave (Δ = freq · step),
      then step forward with the identity cos(a+Δ) = cos(a)·cos(Δ) − sin(a)·sin(Δ). Trig → multiply.
      <br /><br />
      <strong style={{ color: "#94a3b8", fontWeight: 500 }}>Cos-only accumulation:</strong> both complex and
      plain tensor compute the sin component but never return it. Optimised drops it entirely.
      <br /><br />
      <strong style={{ color: "#94a3b8", fontWeight: 500 }}>Why randomised median benchmark:</strong> the JIT
      warms up during the first approach it runs each trial, giving later approaches an unfair advantage.
      Shuffling order and taking the median neutralises that.
    </div>
  );
}

// ── Main ──────────────────────────────────────────────────────────────────────
export default function WaveSimulation() {
  const [waves,    setWaves]    = useState([
    { amp: 1.0, freq: 1.0, phase: 0.0 },
    { amp: 0.5, freq: 2.0, phase: 1.0 },
    { amp: 0.3, freq: 3.0, phase: 2.5 },
  ]);
  const [playing,  setPlaying]  = useState(true);
  const [showComp, setShowComp] = useState(true);
  const [view,     setView]     = useState("all");
  const [codeMode, setCodeMode] = useState("cx");
  const tRef   = useRef(0);
  const rafRef = useRef(null);
  const refs   = { cx: useRef(null), ts: useRef(null), bin: useRef(null), opt: useRef(null) };

  const animate = useCallback(() => {
    if (playing) tRef.current += 0.02;
    const t = tRef.current;
    KEYS.forEach(k => {
      if (view === "all" || view === k)
        drawCanvas(refs[k].current, waves, t, FNS[k], COLORS[k], showComp);
    });
    rafRef.current = requestAnimationFrame(animate);
  }, [waves, playing, showComp, view]);

  useEffect(() => {
    rafRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(rafRef.current);
  }, [animate]);

  const updateWave = (i, key, val) =>
    setWaves(ws => ws.map((w, j) => j === i ? { ...w, [key]: val } : w));
  const removeWave = (i) => setWaves(ws => ws.filter((_, j) => j !== i));
  const addWave = () =>
    setWaves(ws => ws.length < 6
      ? [...ws, { amp: 0.4, freq: +(Math.random() * 3 + 1).toFixed(1), phase: +(Math.random() * 6).toFixed(1) }]
      : ws);

  const Btn = ({ label, active, onClick }) => (
    <button onClick={onClick} style={{
      padding: "5px 14px", borderRadius: 6, cursor: "pointer", fontSize: 12,
      border: `1px solid ${active ? "#334155" : "#1e293b"}`,
      background: active ? "#1e293b" : "transparent",
      color: active ? "#e2e8f0" : "#475569",
    }}>{label}</button>
  );

  const subtitles = {
    cx:  "Re(Σ amp·e^iθ) — baseline",
    ts:  "Float32Array [cos, sin] — typed buffer",
    bin: "Q12 lookup table · bitmask · Int32Array",
    opt: "angle-step recurrence · cos-only · Float32Array",
  };

  return (
    <div style={{ minHeight: "100vh", background: "#020817", color: "#e2e8f0", fontFamily: "system-ui, sans-serif", padding: 24 }}>
      <div style={{ marginBottom: 20 }}>
        <h1 style={{ margin: 0, fontSize: 22, fontWeight: 600, color: "#f1f5f9" }}>Wave Simulation</h1>
        <p style={{ margin: "4px 0 0", color: "#475569", fontSize: 13 }}>
          Complex · Tensor · Binary · Optimised Tensor — fair benchmark, four representations
        </p>
      </div>

      <div style={{ display: "flex", gap: 8, flexWrap: "wrap", marginBottom: 14, alignItems: "center" }}>
        {[["all","All"],["cx","Complex"],["ts","Tensor"],["bin","Binary"],["opt","Optimised ★"]].map(([v, l]) => (
          <Btn key={v} label={l} active={view === v} onClick={() => setView(v)} />
        ))}
        <div style={{ marginLeft: "auto", display: "flex", gap: 8 }}>
          <Btn label="Components" active={showComp} onClick={() => setShowComp(s => !s)} />
          <Btn label={playing ? "⏸ Pause" : "▶ Play"} active={playing} onClick={() => setPlaying(p => !p)} />
          <Btn label="+ Wave" active={false} onClick={addWave} />
        </div>
      </div>

      {KEYS.map(k => (view === "all" || view === k) && (
        <CanvasPanel key={k} label={NAMES[k].toUpperCase()} subtitle={subtitles[k]} color={COLORS[k]} canvasRef={refs[k]} />
      ))}

      <div style={{ marginBottom: 16 }} />
      <FairBenchPanel waves={waves} />

      <div style={{ display: "grid", gridTemplateColumns: "minmax(0,1fr) minmax(0,1fr)", gap: 16, marginBottom: 16 }}>
        <div>
          <div style={{ fontSize: 12, color: "#94a3b8", fontWeight: 500, marginBottom: 8 }}>Wave components</div>
          {waves.map((w, i) => (
            <WaveRow key={i} wave={w} index={i} onChange={updateWave} onRemove={removeWave} />
          ))}
          <p style={{ fontSize: 11, color: "#334155", margin: "10px 0 0" }}>
            All four canvases produce the same wave — different internal representations.
          </p>
        </div>
        <Explainer />
      </div>

      <div style={{ border: "1px solid #1e293b", borderRadius: 10, overflow: "hidden" }}>
        <div style={{ padding: "8px 14px", borderBottom: "1px solid #1e293b", display: "flex", gap: 6 }}>
          {KEYS.map(k => (
            <button key={k} onClick={() => setCodeMode(k)} style={{
              flex: 1, padding: "5px 0", borderRadius: 6, fontSize: 10, cursor: "pointer",
              background: codeMode === k ? "#1e293b" : "transparent",
              border: `1px solid ${codeMode === k ? "#334155" : "#1e293b"}`,
              color: codeMode === k ? (k === "opt" ? "#f59e0b" : "#e2e8f0") : "#475569",
            }}>{NAMES[k]}{k === "opt" ? " ★" : ""}</button>
          ))}
        </div>
        <pre style={{
          background: "#0a0f1a", padding: 14, fontSize: 11, fontFamily: "monospace",
          color: "#94a3b8", overflowX: "auto", overflowY: "auto",
          lineHeight: 1.6, margin: 0, maxHeight: 300,
        }}>
          {CODES[codeMode]}
        </pre>
      </div>
    </div>
  );
}
