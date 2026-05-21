import type { Wave, WaveCombiner } from "../types";
import { CANVAS_HEIGHT, POINT_COUNT } from "./constants";

export function drawCanvas(
  canvas: HTMLCanvasElement | null,
  waves: readonly Wave[],
  t: number,
  fn: WaveCombiner,
  color: string,
  showComponents: boolean,
): void {
  if (!canvas) return;

  const width = canvas.clientWidth || canvas.offsetWidth || 600;
  const dpr = window.devicePixelRatio || 1;
  canvas.width = Math.round(width * dpr);
  canvas.height = Math.round(CANVAS_HEIGHT * dpr);

  const context = canvas.getContext("2d");
  if (!context) return;

  context.setTransform(dpr, 0, 0, dpr, 0, 0);
  context.clearRect(0, 0, width, CANVAS_HEIGHT);
  drawGrid(context, width);

  const maxAmp = waves.reduce((sum, wave) => sum + wave.amp, 0) || 1;
  const scale = (CANVAS_HEIGHT / 2 - 6) / maxAmp;

  if (showComponents) {
    drawComponentWaves(context, waves, t, width, scale);
  }

  context.beginPath();
  context.strokeStyle = color;
  context.lineWidth = 2;

  for (let point = 0; point < POINT_COUNT; point += 1) {
    const x = (point / POINT_COUNT) * width;
    const localTime = (point / POINT_COUNT) * Math.PI * 4 + t;
    const y = CANVAS_HEIGHT / 2 - fn(waves, localTime) * scale;

    if (point === 0) context.moveTo(x, y);
    else context.lineTo(x, y);
  }

  context.stroke();
}

function drawGrid(context: CanvasRenderingContext2D, width: number): void {
  const background = context.createLinearGradient(0, 0, 0, CANVAS_HEIGHT);
  background.addColorStop(0, "#121821");
  background.addColorStop(0.5, "#0f141c");
  background.addColorStop(1, "#0b1017");
  context.fillStyle = background;
  context.fillRect(0, 0, width, CANVAS_HEIGHT);

  drawGridLines(context, width, 24, "rgba(148, 163, 184, 0.08)", 1);
  drawGridLines(context, width, 96, "rgba(148, 163, 184, 0.16)", 1);
  drawHorizontalGridLines(context, width, 20, "rgba(148, 163, 184, 0.08)", 1);

  context.beginPath();
  context.strokeStyle = "rgba(245, 165, 36, 0.24)";
  context.lineWidth = 1.25;
  context.moveTo(0, CANVAS_HEIGHT / 2 + 0.5);
  context.lineTo(width, CANVAS_HEIGHT / 2 + 0.5);
  context.stroke();

  context.beginPath();
  context.strokeStyle = "rgba(255, 255, 255, 0.06)";
  context.lineWidth = 1;
  context.moveTo(0, 0.5);
  context.lineTo(width, 0.5);
  context.moveTo(0, CANVAS_HEIGHT - 0.5);
  context.lineTo(width, CANVAS_HEIGHT - 0.5);
  context.stroke();
}

function drawGridLines(
  context: CanvasRenderingContext2D,
  width: number,
  spacing: number,
  color: string,
  lineWidth: number,
): void {
  context.beginPath();
  context.strokeStyle = color;
  context.lineWidth = lineWidth;

  for (let x = 0; x <= width; x += spacing) {
    context.moveTo(Math.round(x) + 0.5, 0);
    context.lineTo(Math.round(x) + 0.5, CANVAS_HEIGHT);
  }

  context.stroke();
}

function drawHorizontalGridLines(
  context: CanvasRenderingContext2D,
  width: number,
  spacing: number,
  color: string,
  lineWidth: number,
): void {
  context.beginPath();
  context.strokeStyle = color;
  context.lineWidth = lineWidth;

  for (let y = spacing; y < CANVAS_HEIGHT; y += spacing) {
    context.moveTo(0, Math.round(y) + 0.5);
    context.lineTo(width, Math.round(y) + 0.5);
  }

  context.stroke();
}

function drawComponentWaves(
  context: CanvasRenderingContext2D,
  waves: readonly Wave[],
  t: number,
  width: number,
  scale: number,
): void {
  waves.forEach(({ amp, freq, phase }, index) => {
    context.beginPath();
    context.strokeStyle = `hsla(${index * 90 + 180}, 60%, 62%, 0.24)`;
    context.lineWidth = 1;

    for (let point = 0; point < POINT_COUNT; point += 1) {
      const x = (point / POINT_COUNT) * width;
      const localTime = (point / POINT_COUNT) * Math.PI * 4 + t;
      const y = CANVAS_HEIGHT / 2 - amp * Math.cos(freq * localTime + phase) * scale;

      if (point === 0) context.moveTo(x, y);
      else context.lineTo(x, y);
    }

    context.stroke();
  });
}
