import Complex from "complex.js";
import type { Wave } from "../types";

export function cxCombine(waves: readonly Wave[], t: number): number {
  let sum = Complex(0, 0);

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    sum = sum.add(amp * Math.cos(angle), amp * Math.sin(angle));
  }

  return sum.re;
}
