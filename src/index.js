import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const BYTES_PER_PIXEL = 4;

const PAGE_FRACTION = 3;
// const SOURCE_FACTOR = 0.33;

const scale = window.devicePixelRatio;

let { clientWidth, clientHeight } = document.body;

clientWidth = Math.round(clientWidth / PAGE_FRACTION);
clientHeight = Math.round(clientHeight / PAGE_FRACTION);

const width = Math.round(clientWidth * scale);
const height = Math.round(clientHeight * scale);
const numSources = 10;

console.log(numSources);

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");

canvas.style.width = `${clientWidth}px`;
canvas.style.height = `${clientHeight}px`;
canvas.width = width;
canvas.height = height;
context.scale(scale, scale);

const spectrum = Spectrum.new(width, height, numSources);

// for (let i = 0; i < 5; i++) {
//   console.time(`${i}`);

//   const spectrumData = spectrum.data();
//   const spectrumArray = new Uint8ClampedArray(
//     memory.buffer,
//     spectrumData,
//     width * height * BYTES_PER_PIXEL
//   );
//   const spectrumImageData = new ImageData(spectrumArray, width, height);

//   context.putImageData(spectrumImageData, 0, 0);

//   spectrum.tick();
//   spectrum.draw();
//   console.timeEnd(`${i}`);
// }
// canvas.classList.add("show");

const THOUSAND = 1000;

class FPS {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = (1 / delta) * THOUSAND;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
}

const fps = new FPS();

let animationId = null;

const renderLoop = () => {
  // console.time("hi");
  fps.render(); //new

  const spectrumData = spectrum.data();
  const spectrumArray = new Uint8ClampedArray(
    memory.buffer,
    spectrumData,
    width * height * BYTES_PER_PIXEL
  );
  const spectrumImageData = new ImageData(spectrumArray, width, height);

  context.putImageData(spectrumImageData, 0, 0);

  spectrum.tick();
  spectrum.draw();
  // console.timeEnd("hi");

  animationId = window.requestAnimationFrame(renderLoop);
};

// window.requestAnimationFrame(renderLoop);

const isPaused = () => animationId === null;

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", () => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

play();
