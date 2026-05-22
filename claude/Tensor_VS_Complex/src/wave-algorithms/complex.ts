import Complex from "complex.js";
import type { Wave } from "../types";

export type ComplexWaveResult = {
  readonly signal: number;
  readonly complex: {
    readonly re: number;
    readonly im: number;
  };
  readonly magnitude: number;
  readonly phase: number;
};

export function combineAsComplex(
  waves: readonly Wave[],
  t: number,
): ComplexWaveResult {
  let sum = Complex(0, 0);

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    sum = sum.add(amp * Math.cos(angle), amp * Math.sin(angle));
  }

  return {
    signal: sum.re,
    complex: {
      re: sum.re,
      im: sum.im,
    },
    magnitude: sum.abs(),
    phase: sum.arg(),
  };
}

export function cxCombine(waves: readonly Wave[], t: number): number {
  let sum = Complex(0, 0);

  for (const { amp, freq, phase } of waves) {
    const angle = freq * t + phase;
    sum = sum.add(amp * Math.cos(angle), amp * Math.sin(angle));
  }

  return sum.re;
}
