import { Spectrum, SpectrumGL } from "wasm-spectrum";
import FPS from "./utils/fps";

const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;

const MODE_LABELS = {
  wasm: "WebAssembly",
  webgl: "WebGL",
};

const canvasWebgl = document.getElementById("canvas-webgl");
const canvasWasm = document.getElementById("canvas-wasm");
const modeText = document.getElementById("mode");
const widthText = document.getElementById("width");
const setWidth = document.getElementById("set-width");
const heightText = document.getElementById("height");
const setHeight = document.getElementById("set-height");
const numSourcesText = document.getElementById("num-sources");
const setNumSources = document.getElementById("set-num-sources");
const playPauseButton = document.getElementById("play-pause");
const toggleButton = document.getElementById("toggle");

const contextWebgl = canvasWebgl.getContext("webgl");
const contextWasm = canvasWasm.getContext("2d");
contextWasm.scale(DEVICE_SCALE, DEVICE_SCALE);

// const glState = {
//   canvas: canvasWebgl,
//   mode: "webgl",
//   width: Math.round(MAX_WIDTH * 0.8),
//   height: Math.round(MAX_HEIGHT * 0.8),
//   numSources: 20,
// };

const wasmState = {
  canvas: canvasWasm,
  mode: "wasm",
  width: 1280,
  height: 720,
  numSources: 10,
};

let { canvas, mode, width, height, numSources } = wasmState;
let animationId = null;

const setupCanvas = () => {
  widthText.textContent = width;
  heightText.textContent = height;
  numSourcesText.textContent = numSources;

  canvas.style.width = `${width / DEVICE_SCALE}px`;
  canvas.style.height = `${height / DEVICE_SCALE}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }
};

const getNewSpectrumGl = () => {
  setupCanvas();
  return SpectrumGL.new(width, height, numSources, contextWebgl);
};

const getNewSpectrum = () => {
  setupCanvas();
  return Spectrum.new(width, height, numSources, contextWasm);
};

let spectrum = getNewSpectrum();

const restartSpectrum = () => {
  const shouldPlay = !isPaused();

  if (shouldPlay) {
    pause();
  }

  if (mode === "webgl") {
    canvasWasm.classList.remove("show");
    canvasWebgl.classList.add("show");
    canvas = canvasWebgl;

    spectrum = getNewSpectrumGl();
  } else if (mode === "wasm") {
    canvasWebgl.classList.remove("show");
    canvasWasm.classList.add("show");
    canvas = canvasWasm;
    spectrum = getNewSpectrum();
  }

  if (shouldPlay) {
    play();
  }
};

modeText.textContent = MODE_LABELS[mode];

setWidth.min = 100;
setWidth.max = MAX_WIDTH;
setWidth.value = width;
setWidth.addEventListener("change", (e) => {
  const newWidth = e.target.value;
  width = newWidth;
  widthText.textContent = width;
  restartSpectrum();
});

setHeight.min = 100;
setHeight.max = MAX_HEIGHT;
setHeight.value = height;
setHeight.addEventListener("change", (e) => {
  const newHeight = e.target.value;
  height = newHeight;
  heightText.textContent = height;
  restartSpectrum();
});

setNumSources.min = 2;
setNumSources.max = 100;
setNumSources.value = numSources;
setNumSources.addEventListener("change", (e) => {
  const newNumSources = e.target.value;
  numSources = newNumSources;
  numSourcesText.text = numSources;
  restartSpectrum();
});

const fps = new FPS();

const drawFrame = () => {
  fps.render();
  spectrum.draw();
};

const renderLoop = () => {
  drawFrame();
  spectrum.tick();

  animationId = window.requestAnimationFrame(renderLoop);
};

const isPaused = () => animationId === null;

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

toggleButton.addEventListener("click", () => {
  if (mode === "webgl") {
    mode = "wasm";
  } else if (mode === "wasm") {
    mode = "webgl";
  }
  modeText.textContent = MODE_LABELS[mode];
  restartSpectrum();
});

play();
