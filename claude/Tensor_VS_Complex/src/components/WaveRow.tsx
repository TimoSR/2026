import type { Wave, WaveKey } from "../types";

const WAVE_CONTROLS: ReadonlyArray<readonly [WaveKey, number, number, number]> = [
  ["amp", 0.1, 2, 0.05],
  ["freq", 0.1, 6, 0.1],
  ["phase", 0, 6.28, 0.05],
];

type WaveRowProps = {
  wave: Wave;
  index: number;
  onChange: (index: number, key: WaveKey, value: number) => void;
  onRemove: (index: number) => void;
};

export default function WaveRow({ wave, index, onChange, onRemove }: WaveRowProps) {
  return (
    <div className="wave-row">
      <span className="wave-index">{index + 1}</span>

      {WAVE_CONTROLS.map(([key, min, max, step]) => (
        <label key={key} className="wave-control">
          <span>{key}</span>
          <span className="range-line">
            <input
              type="range"
              min={min}
              max={max}
              step={step}
              value={wave[key]}
              onChange={(event) => onChange(index, key, Number(event.target.value))}
            />
            <output>{wave[key].toFixed(1)}</output>
          </span>
        </label>
      ))}

      <button
        type="button"
        className="icon-button"
        aria-label={`Remove wave ${index + 1}`}
        onClick={() => onRemove(index)}
      >
        X
      </button>
    </div>
  );
}
