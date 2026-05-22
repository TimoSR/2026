import type { Wave, WaveCombiner } from "../types";
import { CANVAS_HEIGHT, POINT_COUNT } from "./constants";

const TRACKED_POINT_RATIO = 0.5;
const GRID_DIVISIONS = 12;

export function getTrackedGraphTime(t: number): number {
  return t + TRACKED_POINT_RATIO * Math.PI * 4;
}

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
  drawGrid(context, width, dpr);

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

  drawTrackedPoint(context, waves, t, width, scale, fn, color, dpr);
}

function drawGrid(context: CanvasRenderingContext2D, width: number, dpr: number): void {
  const background = context.createLinearGradient(0, 0, 0, CANVAS_HEIGHT);
  background.addColorStop(0, "#121821");
  background.addColorStop(0.5, "#0f141c");
  background.addColorStop(1, "#0b1017");
  context.fillStyle = background;
  context.fillRect(0, 0, width, CANVAS_HEIGHT);

  drawVerticalGridLines(context, width, GRID_DIVISIONS, "rgba(148, 163, 184, 0.08)", dpr);

  context.beginPath();
  context.strokeStyle = "rgba(245, 165, 36, 0.24)";
  context.lineWidth = getHairlineWidth(dpr);
  context.moveTo(0, snapToDevicePixel(CANVAS_HEIGHT / 2, dpr));
  context.lineTo(width, snapToDevicePixel(CANVAS_HEIGHT / 2, dpr));
  context.stroke();

  context.beginPath();
  context.strokeStyle = "rgba(255, 255, 255, 0.06)";
  context.lineWidth = getHairlineWidth(dpr);
  context.moveTo(0, snapToDevicePixel(0, dpr));
  context.lineTo(width, snapToDevicePixel(0, dpr));
  context.moveTo(0, snapToEndDevicePixel(CANVAS_HEIGHT, dpr));
  context.lineTo(width, snapToEndDevicePixel(CANVAS_HEIGHT, dpr));
  context.stroke();
}

function drawVerticalGridLines(
  context: CanvasRenderingContext2D,
  width: number,
  divisions: number,
  color: string,
  dpr: number,
): void {
  context.beginPath();
  context.strokeStyle = color;
  context.lineWidth = getHairlineWidth(dpr);

  for (let index = 1; index < divisions; index += 1) {
    const x = snapToDevicePixel((width * index) / divisions, dpr);
    context.moveTo(x, 0);
    context.lineTo(x, CANVAS_HEIGHT);
  }

  context.stroke();
}

function getHairlineWidth(dpr: number): number {
  return 1 / dpr;
}

function snapToDevicePixel(value: number, dpr: number): number {
  return (Math.round(value * dpr) + 0.5) / dpr;
}

function snapToEndDevicePixel(value: number, dpr: number): number {
  return (Math.round(value * dpr) - 0.5) / dpr;
}

function drawTrackedPoint(
  context: CanvasRenderingContext2D,
  waves: readonly Wave[],
  t: number,
  width: number,
  scale: number,
  fn: WaveCombiner,
  color: string,
  dpr: number,
): void {
  const x = snapToDevicePixel(width * TRACKED_POINT_RATIO, dpr);
  const localTime = getTrackedGraphTime(t);
  const y = CANVAS_HEIGHT / 2 - fn(waves, localTime) * scale;

  context.beginPath();
  context.strokeStyle = "rgba(242, 244, 247, 0.28)";
  context.lineWidth = getHairlineWidth(dpr);
  context.setLineDash([4, 4]);
  context.moveTo(x, 0);
  context.lineTo(x, CANVAS_HEIGHT);
  context.stroke();
  context.setLineDash([]);

  context.beginPath();
  context.fillStyle = color;
  context.strokeStyle = "#f2f4f7";
  context.lineWidth = 1.5;
  context.arc(x, y, 4, 0, Math.PI * 2);
  context.fill();
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
