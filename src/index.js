import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const BYTES_PER_PIXEL = 4;

const scale = window.devicePixelRatio;

const width = 1280;
const height = 720;
const numSources = 10;

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");

canvas.style.width = `${width / scale}px`;
canvas.style.height = `${height / scale}px`;
canvas.width = width;
canvas.height = height;
context.scale(scale, scale);

const spectrum = Spectrum.new(width, height, numSources);

let animationId = null;

const renderLoop = () => {
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

  animationId = window.requestAnimationFrame(renderLoop);
};

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
