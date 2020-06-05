import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const BYTES_PER_PIXEL = 4;

const scale = window.devicePixelRatio;

let spectrum = null;
let animationId = null;
let width = 1280;
let height = 720;
let numSources = 10;
let maxWidth = document.body.clientWidth * scale;
let maxHeight = document.body.clientHeight * scale;

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");

const initSpectrum = () => {
  canvas.style.width = `${width / scale}px`;
  canvas.style.height = `${height / scale}px`;
  canvas.width = width;
  canvas.height = height;
  context.scale(scale, scale);

  spectrum = Spectrum.new(width, height, numSources);
};

initSpectrum(width, height, numSources);

const restartSpectrum = () => {
  pause();
  initSpectrum();
  drawFrame();
};

const widthText = document.getElementById("width");
widthText.textContent = width;

const setWidth = document.getElementById("set-width");
setWidth.min = 100;
setWidth.max = maxWidth;
setWidth.value = width;
setWidth.addEventListener("change", (e) => {
  const newWidth = e.target.value;
  width = newWidth;
  widthText.textContent = width;
  restartSpectrum();
});

const heightText = document.getElementById("height");
heightText.textContent = height;

const setHeight = document.getElementById("set-height");
setHeight.min = 100;
setHeight.max = maxHeight;
setHeight.value = height;
setHeight.addEventListener("change", (e) => {
  const newHeight = e.target.value;
  height = newHeight;
  heightText.textContent = height;
  restartSpectrum();
});

const setNumSources = document.getElementById("set-num-sources");
setNumSources.min = 2;
setNumSources.value = numSources;
setNumSources.addEventListener("input", (e) => {
  const newNumSources = e.target.value;
  numSources = newNumSources;
  restartSpectrum();
});

const drawFrame = () => {
  const spectrumData = spectrum.data();
  const spectrumArray = new Uint8ClampedArray(
    memory.buffer,
    spectrumData,
    width * height * BYTES_PER_PIXEL
  );
  const spectrumImageData = new ImageData(spectrumArray, width, height);

  context.putImageData(spectrumImageData, 0, 0);

  spectrum.draw();
};

const renderLoop = () => {
  drawFrame();
  spectrum.tick();

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
