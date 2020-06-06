import { Spectrum } from "wasm-spectrum";
import FPS from "./utils/fps";

const scale = window.devicePixelRatio;

let spectrum = null;
let animationId = null;

let numSources = 20;
let maxWidth = document.body.clientWidth * scale;
let maxHeight = document.body.clientHeight * scale;

let width = Math.round(maxWidth * 0.8);
let height = Math.round(maxHeight * 0.8);

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");

const initSpectrum = () => {
  canvas.style.width = `${width / scale}px`;
  canvas.style.height = `${height / scale}px`;
  canvas.width = width;
  canvas.height = height;
  // context.viewport(0, 0, width, height);
  context.scale(scale, scale);
  console.log("hello");
  spectrum = Spectrum.new(width, height, numSources, context);
  console.log("goodbye");
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

const fps = new FPS();

const drawFrame = () => {
  fps.render();
  // const spectrumData = spectrum.data();
  // const spectrumArray = new Uint8ClampedArray(
  //   memory.buffer,
  //   spectrumData,
  //   width * height * BYTES_PER_PIXEL
  // );
  // const spectrumImageData = new ImageData(spectrumArray, width, height);

  // context.putImageData(spectrumImageData, 0, 0);

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
