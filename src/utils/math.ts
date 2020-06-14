/* eslint-disable @typescript-eslint/no-magic-numbers */
const TWO_PI = Math.PI * 2;
const PI_3_2 = Math.PI * 1.5;
const PI_4 = Math.PI / 4;
const PI_2 = Math.PI / 2;

export const atanApprox = (quotient: number): number =>
  (PI_4 + 0.273 * (1 - Math.abs(quotient))) * quotient;

export const atan2Approx = (cos: number, sin: number): number => {
  if (Math.abs(cos) > Math.abs(sin)) {
    if (cos < 0) {
      return atanApprox(sin / cos) + Math.PI;
    }
    if (sin < 0) {
      return atanApprox(sin / cos) + TWO_PI;
    }
    return atanApprox(sin / cos);
  }
  if (sin < 0) {
    return PI_3_2 - atanApprox(cos / sin);
  }
  return PI_2 - atanApprox(cos / sin);
};
