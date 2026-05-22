export default function Explainer() {
  return (
    <section className="panel explainer">
      <h2>Complex numbers as tensors</h2>
      <p>
        <strong>2D real vector:</strong> a complex value z = re + i * im is the
        same state as [x, y], where x = re and y = im.
      </p>
      <p>
        <strong>Rotating wave component:</strong> x = amp * cos(angle) and y =
        amp * sin(angle). The signal is the x-axis projection.
      </p>
      <p>
        <strong>Derived values:</strong> magnitude = sqrt(x^2 + y^2), and phase =
        atan2(y, x). No imaginary-number representation is required.
      </p>
    </section>
  );
}
