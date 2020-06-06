import { Spectrum, SpectrumGL } from "wasm-spectrum";
import FPS from "./utils/fps";

const DEVICE_SCALE = window.devicePixelRatio;

let MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
let MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;

// // let spectrum = null;
// // let animationId = null;
// const canvas = document.getElementById("spectrum-canvas");
// const context = canvas.getContext("webgl");

let stateInitialized = false;

const canvasWebgl = document.getElementById("canvas-webgl");
const contextWebgl = canvasWebgl.getContext("webgl");

const canvasWasm = document.getElementById("canvas-wasm");
const contextWasm = canvasWasm.getContext("2d");
contextWasm.scale(DEVICE_SCALE, DEVICE_SCALE);

const glState = {
  canvas: canvasWebgl,
  mode: "webgl",
  width: Math.round(MAX_WIDTH * 0.8),
  height: Math.round(MAX_HEIGHT * 0.8),
  numSources: 20,
};
// glState.context.viewport(0, 0, glState.width, glState.height);

const wasmState = {
  canvas: canvasWasm,
  mode: "wasm",
  width: 1280,
  height: 720,
  numSources: 10,
};
// wasmState.context.scale(DEVICE_SCALE, DEVICE_SCALE);

// const initialGlWidth = Math.round(MAX_WIDTH * 0.8);
// const initialGlHeight = Math.round(MAX_HEIGHT * 0.8);
// const initialGlNumSources = 20;
// const initialGlContext = canvas.getContext("webgl");

// const initialWasmWidth = 1280;
// const initial

const setupCanvas = () => {
  const { width, height, mode, canvas } = stateInitialized ? state : wasmState;

  canvas.style.width = `${width / DEVICE_SCALE}px`;
  canvas.style.height = `${height / DEVICE_SCALE}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }
};

const getNewSpectrumGl = () => {
  const { width, height, numSources } = stateInitialized ? state : glState;
  setupCanvas();
  return SpectrumGL.new(width, height, numSources, contextWebgl);
};

const getNewSpectrum = () => {
  const { width, height, numSources } = stateInitialized ? state : wasmState;
  setupCanvas();

  return Spectrum.new(width, height, numSources, contextWasm);
};

let state = {
  ...wasmState,
  animationId: null,
  spectrum: getNewSpectrum(),
};

stateInitialized = true;

const restartSpectrum = () => {
  const { mode } = state;
  pause();
  if (mode === "webgl") {
    canvasWasm.classList.remove("show");
    canvasWebgl.classList.add("show");
    state = {
      ...state,
      ...glState,
    };
    state.spectrum = getNewSpectrumGl();
  } else if (mode === "wasm") {
    canvasWebgl.classList.remove("show");
    canvasWasm.classList.add("show");
    state = {
      ...state,
      ...wasmState,
    };
    state.spectrum = getNewSpectrum();
  }
  drawFrame();
};

// setupCanvas();

const widthText = document.getElementById("width");
widthText.textContent = state.width;

const setWidth = document.getElementById("set-width");
setWidth.min = 100;
setWidth.max = MAX_WIDTH;
setWidth.value = state.width;
setWidth.addEventListener("change", (e) => {
  const newWidth = e.target.value;
  state.width = newWidth;
  widthText.textContent = state.width;
  restartSpectrum();
});

const heightText = document.getElementById("height");
heightText.textContent = state.height;

const setHeight = document.getElementById("set-height");
setHeight.min = 100;
setHeight.max = MAX_HEIGHT;
setHeight.value = state.height;
setHeight.addEventListener("change", (e) => {
  const newHeight = e.target.value;
  state.height = newHeight;
  heightText.textContent = state.height;
  restartSpectrum();
});

const setNumSources = document.getElementById("set-num-sources");
setNumSources.min = 2;
setNumSources.value = state.numSources;
setNumSources.addEventListener("input", (e) => {
  const newNumSources = e.target.value;
  state.numSources = newNumSources;
  restartSpectrum();
});

const fps = new FPS();

const drawFrame = () => {
  fps.render();
  state.spectrum.draw();
};

const renderLoop = () => {
  drawFrame();
  state.spectrum.tick();

  state.animationId = window.requestAnimationFrame(renderLoop);
};

const isPaused = () => state.animationId === null;

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(state.animationId);
  state.animationId = null;
};

playPauseButton.addEventListener("click", () => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const toggleButton = document.getElementById("toggle");
toggleButton.addEventListener("click", () => {
  if (state.mode === "webgl") {
    state.mode = "wasm";
  } else if (state.mode === "wasm") {
    state.mode = "webgl";
  }
  restartSpectrum();
});

play();
