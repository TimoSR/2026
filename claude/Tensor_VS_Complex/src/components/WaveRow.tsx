import type { CSSProperties } from "react";
import type { Wave, WaveKey } from "../types";

type WaveRowStyle = CSSProperties & {
  "--wave-color": string;
};

const WAVE_CONTROLS: ReadonlyArray<{
  key: WaveKey;
  label: string;
  min: number;
  max: number;
  step: number;
  precision: number;
}> = [
  { key: "amp", label: "Amp", min: 0.1, max: 2, step: 0.05, precision: 2 },
  { key: "freq", label: "Freq", min: 0.1, max: 6, step: 0.1, precision: 1 },
  { key: "phase", label: "Phase", min: 0, max: 6.28, step: 0.05, precision: 2 },
];

type WaveRowProps = {
  wave: Wave;
  index: number;
  onChange: (index: number, key: WaveKey, value: number) => void;
  onRemove: (index: number) => void;
};

function getWaveColor(index: number): string {
  return `hsl(${index * 90 + 180} 60% 62%)`;
}

export default function WaveRow({ wave, index, onChange, onRemove }: WaveRowProps) {
  const rowStyle: WaveRowStyle = {
    "--wave-color": getWaveColor(index),
  };

  return (
    <div className="wave-row" style={rowStyle}>
      <div className="wave-row-header">
        <span className="wave-title">
          <span className="wave-color" aria-hidden="true" />
          <span className="wave-index">Wave {index + 1}</span>
        </span>
        <button
          type="button"
          className="icon-button remove-wave-button"
          aria-label={`Remove wave ${index + 1}`}
          title={`Remove wave ${index + 1}`}
          onClick={() => onRemove(index)}
        >
          <svg
            aria-hidden="true"
            fill="none"
            height="15"
            viewBox="0 0 24 24"
            width="15"
          >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
          </svg>
        </button>
      </div>

      <div className="wave-controls">
        {WAVE_CONTROLS.map(({ key, label, min, max, step, precision }) => (
          <label key={key} className="wave-control">
            <span>{label}</span>
            <span className="range-line">
              <input
                type="range"
                min={min}
                max={max}
                step={step}
                value={wave[key]}
                onChange={(event) => onChange(index, key, Number(event.target.value))}
              />
              <output>{wave[key].toFixed(precision)}</output>
            </span>
          </label>
        ))}
      </div>
    </div>
  );
}
