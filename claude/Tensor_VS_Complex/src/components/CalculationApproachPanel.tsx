import type { ComplexWaveResult } from "../wave-algorithms/complex";
import type { WaveTensorResult } from "../wave-algorithms/tensor";

type CalculationApproachPanelProps = {
  complexResult: ComplexWaveResult;
  onPlayingChange: () => void;
  playing: boolean;
  sampleTime: number;
  tensorResult: WaveTensorResult;
};

function formatValue(value: number): string {
  return value.toFixed(4);
}

export default function CalculationApproachPanel({
  complexResult,
  onPlayingChange,
  playing,
  sampleTime,
  tensorResult,
}: CalculationApproachPanelProps) {
  const [x, y] = tensorResult.tensor;
  const { re, im } = complexResult.complex;

  return (
    <section className="panel calculation-panel">
      <div className="panel-heading-row">
        <div className="panel-heading">Tracked graph point</div>
        <div className="tracked-actions">
          <div className="tracked-time">t = {formatValue(sampleTime)}</div>
          <button
            type="button"
            className="control-button"
            onClick={onPlayingChange}
          >
            {playing ? "Pause" : "Play"}
          </button>
        </div>
      </div>

      <div className="approach-grid">
        <article className="approach-card">
          <div className="approach-title">Complex calculation</div>
          <div className="formula-line">z = re + i * im</div>
          <div className="derived-grid">
            <div className="derived-tile">
              <strong>{formatValue(complexResult.signal)}</strong>
              <span>signal = re</span>
            </div>
            <div className="derived-tile derived-vector">
              <strong>{formatValue(re)} + i * {formatValue(im)}</strong>
              <span>z</span>
            </div>
            <div className="derived-tile">
              <strong>{formatValue(complexResult.magnitude)}</strong>
              <span>|z|</span>
            </div>
            <div className="derived-tile">
              <strong>{formatValue(complexResult.phase)}</strong>
              <span>arg(z)</span>
            </div>
          </div>
        </article>

        <article className="approach-card">
          <div className="approach-title">Tensor/vector calculation</div>
          <div className="formula-line">v = [x, y]</div>
          <div className="derived-grid">
            <div className="derived-tile">
              <strong>{formatValue(tensorResult.signal)}</strong>
              <span>signal = x</span>
            </div>
            <div className="derived-tile derived-vector">
              <strong>[{formatValue(x)}, {formatValue(y)}]</strong>
              <span>v</span>
            </div>
            <div className="derived-tile">
              <strong>{formatValue(tensorResult.magnitude)}</strong>
              <span>sqrt(x^2 + y^2)</span>
            </div>
            <div className="derived-tile">
              <strong>{formatValue(tensorResult.phase)}</strong>
              <span>atan2(y, x)</span>
            </div>
          </div>
        </article>
      </div>
    </section>
  );
}
