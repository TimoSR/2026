export default function Explainer() {
  return (
    <details className="panel research-panel">
      <summary className="research-summary">
        <div>
          <h2>Research Notes</h2>
          <p>Why tensors can represent the same wave state.</p>
        </div>
        <span aria-hidden="true" />
      </summary>

      <div className="research-body">
        <article className="research-point">
          <span>Why</span>
          <p>
            Test whether wave calculations need complex numbers, or whether the
            same state can be represented with plain real-valued tensors.
          </p>
        </article>

        <article className="research-point">
          <span>What</span>
          <p>
            z = re + i * im maps to v = [x, y], where x = re and y = im. A
            rotating component becomes x = amp * cos(angle), y = amp * sin(angle).
          </p>
        </article>

        <article className="research-point research-point-conclusion">
          <span>Conclusion</span>
          <p>
            The signal is the x-axis projection. Magnitude and phase are derived
            from sqrt(x^2 + y^2) and atan2(y, x). Complex numbers are compact
            notation for 2D real-vector rotation.
          </p>
        </article>
      </div>
    </details>
  );
}
