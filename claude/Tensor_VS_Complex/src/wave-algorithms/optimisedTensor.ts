import type { Wave } from "../types";

const outputBuffer = new Float32Array(1);

export function optCombine(
  waves: readonly Wave[],
  t: number,
  step = 0.01,
): number {
  outputBuffer[0] = 0;

  for (const { amp, freq, phase } of waves) {
    const initialAngle = freq * t + phase;
    const angleDelta = freq * step;
    const cosDelta = Math.cos(angleDelta);
    const sinDelta = Math.sin(angleDelta);

    let cosAngle = Math.cos(initialAngle);
    let sinAngle = Math.sin(initialAngle);

    outputBuffer[0] += amp * cosAngle;

    const nextCos = cosAngle * cosDelta - sinAngle * sinDelta;
    const nextSin = sinAngle * cosDelta + cosAngle * sinDelta;
    cosAngle = nextCos;
    sinAngle = nextSin;
  }

  return outputBuffer[0];
}
