export default function Explainer() {
  return (
    <section className="panel explainer">
      <h2>What makes optimised faster</h2>
      <p>
        <strong>Angle-step recurrence:</strong> compute the delta once per wave and
        step forward with the cosine angle-addition identity.
      </p>
      <p>
        <strong>Cos-only accumulation:</strong> complex and plain tensor compute the
        sine component but never return it. The optimised variant avoids that
        returned-output mismatch.
      </p>
      <p>
        <strong>Randomised median benchmark:</strong> the run order is shuffled and
        the median is reported so warm-up effects are less likely to dominate the
        result.
      </p>
    </section>
  );
}
