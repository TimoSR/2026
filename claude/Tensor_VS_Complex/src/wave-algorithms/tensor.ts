import type { Wave } from "../types";

const tensorBuffer = new Float32Array(2);

export type Tensor2 = readonly [x: number, y: number];

export type WaveTensorResult = {
  readonly signal: number;
  readonly tensor: Tensor2;
  readonly magnitude: number;
  readonly phase: number;
};

export function combineAsTensor(
  waves: readonly Wave[],
  t: number,
): WaveTensorResult {
  tensorBuffer[0] = 0;
  tensorBuffer[1] = 0;

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    tensorBuffer[0] += amp * Math.cos(angle);
    tensorBuffer[1] += amp * Math.sin(angle);
  }

  const x = tensorBuffer[0];
  const y = tensorBuffer[1];

  return {
    signal: x,
    tensor: [x, y],
    magnitude: Math.hypot(x, y),
    phase: Math.atan2(y, x),
  };
}

export function tsCombine(waves: readonly Wave[], t: number): number {
  return combineAsTensor(waves, t).signal;
}
