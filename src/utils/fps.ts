const THOUSAND = 1000;

export default class FPS {
  node: HTMLElement;
  frames: number[];
  lastFrameTimeStamp: number;

  constructor() {
    this.node = document.getElementById("fps") as HTMLElement;
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
    this.node.textContent = "FPS: ";
  }

  render(): void {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const frameLength = (1 / delta) * THOUSAND;

    // Save only the latest 100 timings.
    this.frames.push(frameLength);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    const fps =
      this.frames.reduce((sum, frame) => sum + frame) / this.frames.length;

    // Render the statistics.
    this.node.textContent = `FPS: ${Math.round(fps)}`;
  }
}
