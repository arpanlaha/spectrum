import { Spectrum, SpectrumGL } from "wasm-spectrum";
import FPS from "./utils/fps";

const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;
const WEBGL_SCALE = 1;
const WASM_SCALE = 0.4;

type Mode = "wasm" | "webgl";

interface Modes {
  wasm: string;
  webgl: string;
}

const MODE_LABELS: Modes = {
  wasm: "WebAssembly",
  webgl: "WebGL",
};

const canvasWebgl = document.getElementById(
  "canvas-webgl"
) as HTMLCanvasElement;
const canvasWasm = document.getElementById("canvas-wasm") as HTMLCanvasElement;
const controls = document.getElementById("controls") as HTMLDivElement;
const playPauseButton = document.getElementById(
  "play-pause"
) as HTMLButtonElement;
const toggleButton = document.getElementById("toggle") as HTMLButtonElement;
const modeText = document.getElementById("mode") as HTMLSpanElement;
const widthText = document.getElementById("width") as HTMLSpanElement;
const setWidth = document.getElementById("set-width") as HTMLInputElement;
const heightText = document.getElementById("height") as HTMLSpanElement;
const setHeight = document.getElementById("set-height") as HTMLInputElement;
const numSourcesText = document.getElementById(
  "num-sources"
) as HTMLSpanElement;
const setNumSources = document.getElementById(
  "set-num-sources"
) as HTMLInputElement;
const movementSpeedText = document.getElementById(
  "movement-speed"
) as HTMLSpanElement;
const setMovementSpeed = document.getElementById(
  "set-movement-speed"
) as HTMLInputElement;
const colorSpeedText = document.getElementById(
  "color-speed"
) as HTMLSpanElement;
const setColorSpeed = document.getElementById(
  "set-color-speed"
) as HTMLInputElement;

const contextWebgl = canvasWebgl.getContext("webgl") as WebGLRenderingContext;
const contextWasm = canvasWasm.getContext("2d") as CanvasRenderingContext2D;
contextWasm.scale(DEVICE_SCALE, DEVICE_SCALE);

interface InitialState {
  canvas: HTMLCanvasElement;
  width: number;
  height: number;
  numSources: number;
  movementSpeed: number;
  colorSpeed: number;
}

interface State extends InitialState {
  spectrum: Spectrum | SpectrumGL;
}

const initialStates: Record<Mode, InitialState> = {
  webgl: {
    canvas: canvasWebgl,
    width: Math.round(MAX_WIDTH * WEBGL_SCALE),
    height: Math.round(MAX_HEIGHT * WEBGL_SCALE),
    numSources: 16,
    movementSpeed: 1,
    colorSpeed: 0.2,
  },
  wasm: {
    canvas: canvasWasm,
    width: Math.round(MAX_WIDTH * WASM_SCALE),
    height: Math.round(MAX_HEIGHT * WASM_SCALE),
    numSources: 10,
    movementSpeed: 2,
    colorSpeed: 0.4,
  },
};

const getInitialState = (mode: Mode): State => {
  const {
    canvas,
    width,
    height,
    numSources,
    movementSpeed,
    colorSpeed,
  } = initialStates[mode];

  widthText.textContent = width.toString();
  setWidth.value = width.toString();

  heightText.textContent = height.toString();
  setHeight.value = height.toString();

  numSourcesText.textContent = numSources.toString();
  setNumSources.value = numSources.toString();

  movementSpeedText.textContent = movementSpeed.toString();
  setMovementSpeed.value = movementSpeed.toString();

  colorSpeedText.textContent = colorSpeed.toString();
  setColorSpeed.value = colorSpeed.toString();

  canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
  canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }

  const spectrum =
    mode === "webgl"
      ? SpectrumGL.new(width, height, numSources, contextWebgl)
      : Spectrum.new(width, height, numSources, contextWasm);

  return {
    canvas,
    width,
    height,
    numSources,
    spectrum,
    movementSpeed,
    colorSpeed,
  };
};

let mode: Mode = "webgl";

let {
  canvas,
  width,
  height,
  numSources,
  spectrum,
  movementSpeed,
  colorSpeed,
} = getInitialState(mode);

let animationId: number | null = null;

const setupCanvas = (): void => {
  widthText.textContent = width.toString();
  setWidth.value = width.toString();

  heightText.textContent = height.toString();
  setHeight.value = height.toString();

  numSourcesText.textContent = numSources.toString();
  setNumSources.value = numSources.toString();

  movementSpeedText.textContent = movementSpeed.toString();
  setMovementSpeed.value = movementSpeed.toString();

  colorSpeedText.textContent = colorSpeed.toString();
  setColorSpeed.value = colorSpeed.toString();

  canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
  canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }
};

const getNewSpectrumGl = (): SpectrumGL => {
  setupCanvas();
  return SpectrumGL.new(width, height, numSources, contextWebgl);
};

const getNewSpectrum = (): Spectrum => {
  setupCanvas();
  return Spectrum.new(width, height, numSources, contextWasm);
};

let fps = new FPS();

const restartSpectrum = (): void => {
  const shouldPlay = !isPaused();

  if (shouldPlay) {
    pause();
  }

  fps = new FPS();

  if (mode === "webgl") {
    canvasWasm.classList.add("hide");
    canvasWebgl.classList.remove("hide");

    spectrum = getNewSpectrumGl();
  } else if (mode === "wasm") {
    canvasWebgl.classList.add("hide");
    canvasWasm.classList.remove("hide");

    spectrum = getNewSpectrum();
  }

  if (shouldPlay) {
    play();
  }
};

modeText.textContent = MODE_LABELS[mode];

setWidth.min = "100";
setWidth.max = MAX_WIDTH.toString();
setWidth.value = width.toString();
setWidth.addEventListener("change", (e) => {
  const newWidth = (e.target as HTMLInputElement).value;
  width = parseInt(newWidth);
  widthText.textContent = width.toString();
  restartSpectrum();
});

setHeight.min = "100";
setHeight.max = MAX_HEIGHT.toString();
setHeight.value = height.toString();
setHeight.addEventListener("change", (e) => {
  const newHeight = (e.target as HTMLInputElement).value;
  height = parseInt(newHeight);
  heightText.textContent = height.toString();
  restartSpectrum();
});

setNumSources.min = "2";
// iOS Safari is dumb and has a limited number of fragment shader uniforms
setNumSources.max = Math.min(
  100,
  contextWebgl.getParameter(contextWebgl.MAX_FRAGMENT_UNIFORM_VECTORS)
).toString();
setNumSources.value = numSources.toString();
setNumSources.addEventListener("change", (e) => {
  const newNumSources = (e.target as HTMLInputElement).value;
  numSources = parseInt(newNumSources);
  numSourcesText.textContent = numSources.toString();
  restartSpectrum();
});

setMovementSpeed.min = "0.1";
setMovementSpeed.max = "5";
setMovementSpeed.step = "0.1";
setMovementSpeed.value = movementSpeed.toString();
setMovementSpeed.addEventListener("change", (e) => {
  const newMovementSpeed = (e.target as HTMLInputElement).value;
  movementSpeed = parseInt(newMovementSpeed);
  movementSpeedText.textContent = movementSpeed.toString();
  restartSpectrum();
});

setColorSpeed.min = "0.01";
setColorSpeed.max = "1";
setColorSpeed.step = "0.01";
setColorSpeed.value = colorSpeed.toString();
setColorSpeed.addEventListener("change", (e) => {
  const newColorSpeed = (e.target as HTMLInputElement).value;
  colorSpeed = parseInt(newColorSpeed);
  colorSpeedText.textContent = colorSpeed.toString();
  restartSpectrum();
});

const drawFrame = (): void => {
  spectrum.draw();
  fps.render();
};

const renderLoop = (): void => {
  drawFrame();
  spectrum.tick();

  animationId = window.requestAnimationFrame(renderLoop);
};

const isPaused = (): boolean => animationId === null;

const play = (): void => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = (): void => {
  if (animationId !== null) {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
  }
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

  const newState = initialStates[mode];
  width = newState.width;
  height = newState.height;
  canvas = newState.canvas;
  numSources = newState.numSources;
  movementSpeed = newState.movementSpeed;
  colorSpeed = newState.colorSpeed;

  modeText.textContent = MODE_LABELS[mode];
  restartSpectrum();
});

controls.classList.remove("hide");
play();
