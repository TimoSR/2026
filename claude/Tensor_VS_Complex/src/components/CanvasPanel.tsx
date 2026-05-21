import type { CSSProperties, RefObject } from "react";
import { CANVAS_HEIGHT } from "../core/constants";

type CSSVariableStyle = CSSProperties & Record<`--${string}`, string | number>;

type CanvasPanelProps = {
  label: string;
  subtitle: string;
  color: string;
  canvasRef: RefObject<HTMLCanvasElement | null>;
};

export default function CanvasPanel({ label, subtitle, color, canvasRef }: CanvasPanelProps) {
  return (
    <article className="canvas-panel">
      <header className="canvas-panel-header">
        <span
          className="canvas-color"
          style={{ "--series-color": color } as CSSVariableStyle}
        />
        <span
          className="canvas-title"
          style={{ "--series-color": color } as CSSVariableStyle}
        >
          {label}
        </span>
        <span className="canvas-subtitle">{subtitle}</span>
      </header>
      <canvas
        ref={canvasRef}
        height={CANVAS_HEIGHT}
        className="wave-canvas"
        aria-label={`${label} wave preview`}
      />
    </article>
  );
}
