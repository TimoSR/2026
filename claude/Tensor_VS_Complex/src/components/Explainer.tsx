export default function Explainer() {
  return (
    <section className="panel explainer">
      <h2>Research conclusion</h2>
      <p>
        <strong>Why:</strong> the question was whether wave calculations need
        complex numbers, or whether the same state can be represented with plain
        real-valued tensors.
      </p>
      <p>
        <strong>What we tested:</strong> a complex value z = re + i * im was
        represented as the 2D vector [x, y], where x = re and y = im. A rotating
        wave component becomes x = amp * cos(angle) and y = amp * sin(angle).
      </p>
      <p>
        <strong>Conclusion:</strong> the signal is just the x-axis projection.
        Magnitude is sqrt(x^2 + y^2), and phase is atan2(y, x). No imaginary
        number representation is required; complex numbers are syntactic
        compression for 2D real-vector rotation.
      </p>
    </section>
  );
}
