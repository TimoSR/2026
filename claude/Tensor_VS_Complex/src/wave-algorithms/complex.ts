import type { Wave } from "../types";

export function cxCombine(waves: readonly Wave[], t: number): number {
  let re = 0;
  let im = 0;

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    re += amp * Math.cos(angle);
    im += amp * Math.sin(angle);
  }

  return re;
}
