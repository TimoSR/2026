import type { CSSProperties } from "react";

type CSSVariableStyle = CSSProperties & Record<`--${string}`, string | number>;

type DotChartProps = {
  times?: readonly number[];
  color: string;
};

export default function DotChart({ times, color }: DotChartProps) {
  if (!times || times.length === 0) {
    return <div className="dot-chart dot-chart-empty" />;
  }

  const sorted = [...times].sort((a, b) => a - b);
  const min = sorted[0];
  const range = sorted[sorted.length - 1] - min || 1;

  return (
    <div className="dot-chart" aria-label="Trial timing distribution">
      {sorted.map((value, index) => (
        <span
          key={`${value}-${index}`}
          className="trial-dot"
          title={`${value.toFixed(2)}ms`}
          style={{
            "--dot-color": color,
            "--dot-opacity": (0.3 + ((value - min) / range) * 0.7).toFixed(2),
          } as CSSVariableStyle}
        />
      ))}
    </div>
  );
}
