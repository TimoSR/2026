import type { CSSProperties, KeyboardEvent, RefObject } from "react";
import { CANVAS_HEIGHT } from "../core/constants";

type CSSVariableStyle = CSSProperties & Record<`--${string}`, string | number>;

type CanvasPanelProps = {
  label: string;
  subtitle: string;
  color: string;
  canvasRef: RefObject<HTMLCanvasElement | null>;
  selected: boolean;
  onSelect: () => void;
};

export default function CanvasPanel({
  label,
  subtitle,
  color,
  canvasRef,
  selected,
  onSelect,
}: CanvasPanelProps) {
  const handleKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    if (event.key !== "Enter" && event.key !== " ") return;

    event.preventDefault();
    onSelect();
  };

  return (
    <article
      aria-label={`Show ${label} implementation code`}
      aria-pressed={selected}
      className={`canvas-panel canvas-panel-selectable${selected ? " is-selected" : ""}`}
      onClick={onSelect}
      onKeyDown={handleKeyDown}
      role="button"
      tabIndex={0}
    >
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
