const THOUSAND = 1000;

/**
 * Tracks frames per second, requiring that the `render` method be called on every frame.
 *
 * Derived from the [Rust and WebAssembly Game of Life Tutorial - Time Profiling section](https://rustwasm.github.io/book/game-of-life/time-profiling.html).
 */
export default class FPS {
  /**
   * The HTML element displaying the information.
   */
  node: HTMLElement;

  /**
   * A list of frame lengths tracked by the FPS.
   */
  frames: number[];

  /**
   * THe time of the last frame.
   */
  lastFrameTimeStamp: number;

  constructor() {
    this.node = document.getElementById("fps") as HTMLElement;
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
    this.node.textContent = "Waiting to start...";
  }

  /**
   * Finds the time since the last frame was recorded, updating the internal list of frames and the displayed FPS count.
   */
  render(): void {
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const frameLength = (1 / delta) * THOUSAND;

    this.frames.push(frameLength);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    this.node.textContent = `${Math.round(
      this.frames.reduce((sum, frame) => sum + frame) / this.frames.length
    )} FPS`;
  }
}
