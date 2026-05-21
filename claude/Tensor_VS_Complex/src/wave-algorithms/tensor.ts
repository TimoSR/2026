import type { Wave } from "../types";

const tensorBuffer = new Float32Array(2);

export function tsCombine(waves: readonly Wave[], t: number): number {
  tensorBuffer[0] = 0;
  tensorBuffer[1] = 0;

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    tensorBuffer[0] += amp * Math.cos(angle);
    tensorBuffer[1] += amp * Math.sin(angle);
  }

  return tensorBuffer[0];
}
