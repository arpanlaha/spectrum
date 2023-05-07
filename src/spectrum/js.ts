/* eslint-disable @typescript-eslint/no-magic-numbers */

import { atan2Approx } from "../utils";

const TWO_PI = Math.PI * 2;
const PI_3 = Math.PI / 3;
const PI_2_3 = 2 * PI_3;
const PI_4_3 = 2 * PI_2_3;
const PI_5_3 = 5 * PI_3;
const U8_MAX = 255;
const MOVEMENT_SPEED_FACTOR = 0.2;
const COLOR_SPEED_FACTOR = 0.002;
const SOURCE_DROPOFF_FACTOR = 0.01;

/**
 * Wrapper of three numbers in [0, 255] corresponding to RGB for a single pixel.
 */
type RGB = [number, number, number];

/**
 * Value in [0, 2π) corresponding to a hue value (in radians) in the HSL color space.
 */
class Hue {
  /**
   * The internal Hue value.
   */
  hue: number;

  constructor(hue: number) {
    this.hue = hue;
  }

  /**
   * Increments the internal value by the specificied delta.
   *
   * If the new value lies outside the valid Hue range, it is adjusted accordingly by one period.
   *
   * @param dh the desired change to the internal value.
   */
  tick(dh: number): void {
    this.hue = (this.hue + dh) % TWO_PI;
  }

  /**
   * Converts the Hue to its corresponding RGB value.
   *
   * Sets saturation to 100% and lightness to 50% to get the Hue's truest color value.
   *
   * Derived from [RapidTables HSL to RGB color conversion](https://www.rapidtables.com/convert/color/hsl-to-rgb.html).
   */
  toRgb(): RGB {
    if (this.hue < Math.PI) {
      if (this.hue < PI_3) {
        return [U8_MAX, (U8_MAX * this.hue) / PI_3, 0];
      }
      if (this.hue < PI_2_3) {
        return [U8_MAX * (2 - this.hue / PI_3), U8_MAX, 0];
      }
      return [0, U8_MAX, U8_MAX * (this.hue / PI_3 - 2)];
    }
    if (this.hue < PI_4_3) {
      return [0, U8_MAX * (4 - this.hue / PI_3), U8_MAX];
    }
    if (this.hue < PI_5_3) {
      return [U8_MAX * (this.hue / PI_3 - 4), 0, U8_MAX];
    }
    return [U8_MAX, 0, U8_MAX * (6 - this.hue / PI_3)];
  }
}

class Source {
  /**
   * The x-coordinate of the Source in the Spectrum canvas.
   */
  x: number;

  /**
   * The y-coordinate of the Source in the Spectrum canvas.
   */
  y: number;

  /**
   * The rate of movement in the x direction.
   */
  dx: number;

  /**
   * The rate of movement in the y direction.
   */
  dy: number;

  /**
   * The rate of change in the Source's Hue.
   */
  dh: number;

  /**
   * The cosine of the internal Hue value.
   */
  hueCos: number;

  /**
   * The sine of the internal Hue value.
   */
  hueSin: number;

  /**
   * The internal Hue value of the Source.
   */
  readonly hue: Hue;

  /**
   * The width of the Spectrum canvas.
   */
  readonly canvasWidth: number;

  /**
   * The height of the spectrum canvas:
   */
  readonly canvasHeight: number;

  private readonly _dxRandom: number;
  private readonly _dyRandom: number;
  private readonly _dhRandom: number;

  constructor(
    canvasWidth: number,
    canvasHeight: number,
    movementSpeed: number,
    colorSpeed: number
  ) {
    const transformedMovementSpeed = movementSpeed * MOVEMENT_SPEED_FACTOR;
    const transformedColorSpeed = colorSpeed * COLOR_SPEED_FACTOR;

    this.x = Math.random() * canvasWidth;
    this.y = Math.random() * canvasHeight;
    this.hue = new Hue(Math.random() * TWO_PI);
    this.canvasWidth = canvasWidth;
    this.canvasHeight = canvasHeight;

    this._dxRandom = Math.random();
    this._dyRandom = Math.random();
    this._dhRandom = Math.random();

    this.dx =
      this._dxRandom * transformedMovementSpeed - transformedMovementSpeed / 2;
    this.dy =
      this._dyRandom * transformedMovementSpeed - transformedMovementSpeed / 2;
    this.dh =
      this._dhRandom * transformedColorSpeed - transformedColorSpeed / 2;

    this.hueCos = Math.cos(this.hue.hue);
    this.hueSin = Math.sin(this.hue.hue);
  }

  updateMovementSpeed(movementSpeed: number): void {
    const transformedMovementSpeed = movementSpeed * MOVEMENT_SPEED_FACTOR;

    this.dx =
      Math.sign(this.dx) *
      Math.abs(
        this._dxRandom * transformedMovementSpeed - transformedMovementSpeed / 2
      );
    this.dy =
      Math.sign(this.dy) *
      Math.abs(
        this._dyRandom * transformedMovementSpeed - transformedMovementSpeed / 2
      );
  }

  updateColorSpeed(colorSpeed: number): void {
    const transformedColorSpeed = colorSpeed * COLOR_SPEED_FACTOR;
    this.dh =
      Math.sign(this.dh) *
      Math.abs(
        this._dhRandom * transformedColorSpeed - transformedColorSpeed / 2
      );
  }

  /**
   * Increments the Source by one frame.
   *
   * The internal hue is incremented by the Source's `dh` value.
   *
   * The Source's position is incremented by `dx` and `dy`, with border collisions behaving as a bounce.
   */
  tick(): void {
    this.hue.tick(this.dh);
    this.hueCos = Math.cos(this.hue.hue);
    this.hueSin = Math.sin(this.hue.hue);

    this.x += this.dx;
    this.y += this.dy;

    if (this.x <= 0) {
      this.x *= -1;
      this.dx *= -1;
    } else if (this.x >= this.canvasWidth) {
      this.x = this.canvasWidth - (this.x - this.canvasWidth);
      this.dx *= -1;
    }

    if (this.y <= 0) {
      this.y *= -1;
      this.dy *= -1;
    } else if (this.y >= this.canvasHeight) {
      this.y = this.canvasHeight - (this.y - this.canvasHeight);
      this.dy *= -1;
    }
  }
}

export default class SpectrumJS {
  /**
   * The width of the Spectrum canvas.
   */
  width: number;

  /**
   * The height of the Spectrum canvas.
   */
  height: number;

  /**
   * A vector containing the Spectrum's sources.
   */
  sources: Source[];

  sourceDropoff: number;

  /**
   * The Spectrum's pixel data.
   */
  data: Uint8ClampedArray;

  /**
   * The `2d` context belonging to the Spectrum's canvas.
   */
  context: CanvasRenderingContext2D;

  constructor(
    width: number,
    height: number,
    numSources: number,
    movementSpeed: number,
    colorSpeed: number,
    sourceDropoff: number,
    canvas: HTMLCanvasElement
  ) {
    this.width = width;
    this.height = height;
    this.sources = Array.from(
      { length: numSources },
      () => new Source(width, height, movementSpeed, colorSpeed)
    );
    this.sourceDropoff = Math.pow(sourceDropoff * SOURCE_DROPOFF_FACTOR, 2);
    this.data = new Uint8ClampedArray(4 * width * height);
    this.context = canvas.getContext("2d") as CanvasRenderingContext2D;

    this.draw();
  }

  /**
   * Wraps the SpectrumJS constructor.
   * @param width the width of the SpectrumJS.
   * @param height the height of the SpectrumJS
   * @param numSources the number of Sources.
   * @param movementSpeed the range of Source movement speed.
   * @param colorSpeed the range of Source Hue speed.
   * @param canvas the canvas element to draw to.
   */
  static new(
    width: number,
    height: number,
    numSources: number,
    movementSpeed: number,
    colorSpeed: number,
    sourceDropoff: number,
    canvas: HTMLCanvasElement
  ): SpectrumJS {
    return new SpectrumJS(
      width,
      height,
      numSources,
      movementSpeed,
      colorSpeed,
      sourceDropoff,
      canvas
    );
  }

  /**
   * Draws to the Spectrum canvas, using the Spectrum's context to put the resulting ImageData.
   *
   * Assigns Hues to each pixel based off of an average inverse square distance weighting across all Sources.
   *
   * As hue in HSL is a circular/periodic metric, a numerical average is inaccurate - instead, hue is broken into sine and cosine components which are summed and reconstructed into the resulting Hue.
   */
  draw(): void {
    for (let y = 0; y < this.height; y++) {
      const y_by_width = y * this.width;

      for (let x = 0; x < this.width; x++) {
        let distFactorInverseSum = 0;

        let [hueVectorCos, hueVectorSin] = [0, 0];
        this.sources.forEach((source) => {
          const distFactor =
            Math.pow(x - source.x, 2) + Math.pow(y - source.y, 2) + 1;
          hueVectorCos += source.hueCos / distFactor;
          hueVectorSin += source.hueSin / distFactor;

          distFactorInverseSum += 1 / distFactor;
        });

        distFactorInverseSum = Math.min(distFactorInverseSum, 1);
        const alpha = Math.round(
          U8_MAX * Math.pow(distFactorInverseSum, this.sourceDropoff)
        );

        const start = (x + y_by_width) * 4;
        const [r, g, b] = new Hue(
          atan2Approx(hueVectorCos, hueVectorSin)
        ).toRgb();

        this.data[start] = r;
        this.data[start + 1] = g;
        this.data[start + 2] = b;
        this.data[start + 3] = alpha;
      }
    }

    this.context.putImageData(
      new ImageData(this.data, this.width, this.height),
      0,
      0
    );
  }

  /**
   * Increments all of the Spectrum's sources by one frame.
   */
  tick(): void {
    for (const source of this.sources) {
      source.tick();
    }
  }

  updateMovementSpeed(movementSpeed: number): void {
    for (const source of this.sources) {
      source.updateMovementSpeed(movementSpeed);
    }
  }

  updateColorSpeed(colorSpeed: number): void {
    for (const source of this.sources) {
      source.updateColorSpeed(colorSpeed);
    }
  }

  updateSourceDropoff(sourceDropoff: number): void {
    this.sourceDropoff = Math.pow(sourceDropoff * SOURCE_DROPOFF_FACTOR, 2);
  }
}
