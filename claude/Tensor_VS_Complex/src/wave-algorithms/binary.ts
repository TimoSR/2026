import type { Wave } from "../types";

const Q = 4096;
const TABLE_LENGTH = 1024;
const TABLE_MASK = TABLE_LENGTH - 1;

const cosTable = new Int32Array(TABLE_LENGTH);
const sinTable = new Int32Array(TABLE_LENGTH);

for (let index = 0; index < TABLE_LENGTH; index += 1) {
  const angle = (index / TABLE_LENGTH) * 2 * Math.PI;
  cosTable[index] = Math.round(Math.cos(angle) * Q);
  sinTable[index] = Math.round(Math.sin(angle) * Q);
}

const packed = new Int32Array(2);

export function binCombine(waves: readonly Wave[], t: number): number {
  packed[0] = 0;
  packed[1] = 0;

  for (const { amp, freq, phase } of waves) {
    let normalized = ((freq * t + phase) / (2 * Math.PI)) % 1;
    if (normalized < 0) normalized += 1;

    const tableIndex = (normalized * TABLE_LENGTH) & TABLE_MASK;
    const ampQ = Math.round(amp * Q);

    packed[0] += (ampQ * cosTable[tableIndex]) >> 12;
    packed[1] += (ampQ * sinTable[tableIndex]) >> 12;
  }

  return packed[0] / Q;
}
