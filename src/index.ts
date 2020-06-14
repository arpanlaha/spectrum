import { SpectrumGL, SpectrumWasm } from "wasm-spectrum";
import { SpectrumJS } from "./spectrum";
import FPS from "./utils/fps";

const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;
const WEBGL_SCALE = 1;
const WASM_SCALE = 0.4;
const JS_SCALE = 0.25;
const MOVEMENT_SPEED_FACTOR = 0.2;
const COLOR_SPEED_FACTOR = 0.005;

type Mode = "webgl" | "wasm" | "js";

interface Spectrum {
  draw: () => void;
  tick: () => void;
}

const canvasWebgl = document.getElementById(
  "canvas-webgl"
) as HTMLCanvasElement;
const canvas2d = document.getElementById("canvas-wasm") as HTMLCanvasElement;
const controls = document.getElementById("controls") as HTMLDivElement;
const playPauseIcon = document.getElementById(
  "play-pause-icon"
) as HTMLImageElement;
const downloadLink = document.getElementById(
  "download-link"
) as HTMLAnchorElement;
const modeWebgl = document.getElementById("mode-webgl") as HTMLDivElement;
const modeWasm = document.getElementById("mode-wasm") as HTMLDivElement;
const modeJs = document.getElementById("mode-js") as HTMLDivElement;
const modeLock = document.getElementById("mode-lock") as HTMLDivElement;
const modeUnlock = document.getElementById("mode-unlock") as HTMLDivElement;
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
const collapse = document.getElementById("collapse") as HTMLImageElement;
const expand = document.getElementById("expand") as HTMLImageElement;

const contextWebgl = canvasWebgl.getContext("webgl", {
  preserveDrawingBuffer: true,
}) as WebGLRenderingContext;
const context2d = canvas2d.getContext("2d") as CanvasRenderingContext2D;
context2d.scale(DEVICE_SCALE, DEVICE_SCALE);

interface InitialState {
  canvas: HTMLCanvasElement;
  width: number;
  height: number;
  numSources: number;
  movementSpeed: number;
  colorSpeed: number;
}

interface State extends InitialState {
  spectrum: Spectrum;
}

/* eslint-disable @typescript-eslint/no-magic-numbers */
const initialStates: Record<Mode, InitialState> = {
  webgl: {
    canvas: canvasWebgl,
    width: Math.round(MAX_WIDTH * WEBGL_SCALE),
    height: Math.round(MAX_HEIGHT * WEBGL_SCALE),
    numSources: Math.min(
      20,
      contextWebgl.getParameter(contextWebgl.MAX_FRAGMENT_UNIFORM_VECTORS)
    ),
    movementSpeed: 40,
    colorSpeed: 20,
  },
  wasm: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * WASM_SCALE),
    height: Math.round(MAX_HEIGHT * WASM_SCALE),
    numSources: 10,
    movementSpeed: 40,
    colorSpeed: 40,
  },
  js: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * JS_SCALE),
    height: Math.round(MAX_HEIGHT * JS_SCALE),
    numSources: 10,
    movementSpeed: 20,
    colorSpeed: 40,
  },
};
/* eslint-enable @typescript-eslint/no-magic-numbers */

const spectrumInitializers = {
  webgl: SpectrumGL,
  wasm: SpectrumWasm,
  js: SpectrumJS,
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

  const spectrum = spectrumInitializers[mode].new(
    width,
    height,
    numSources,
    movementSpeed * MOVEMENT_SPEED_FACTOR,
    colorSpeed * COLOR_SPEED_FACTOR,
    mode === "webgl" ? canvasWebgl : canvas2d
  );

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
let lockParameters = false;

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

const getNewSpectrum = (): Spectrum => {
  setupCanvas();
  return spectrumInitializers[mode].new(
    width,
    height,
    numSources,
    movementSpeed * MOVEMENT_SPEED_FACTOR,
    colorSpeed * COLOR_SPEED_FACTOR,
    mode === "webgl" ? canvasWebgl : canvas2d
  );
};

let fps = new FPS();

const restartSpectrum = (): void => {
  const shouldPlay = !isPaused();

  if (shouldPlay) {
    pause();
  }

  fps = new FPS();

  if (mode === "webgl") {
    canvas2d.classList.add("hide");
    canvasWebgl.classList.remove("hide");
  } else {
    canvasWebgl.classList.add("hide");
    canvas2d.classList.remove("hide");
  }

  spectrum = getNewSpectrum();

  if (shouldPlay) {
    play();
  }
};

const renderLoop = (): void => {
  spectrum.draw();
  fps.render();
  spectrum.tick();

  animationId = window.requestAnimationFrame(renderLoop);
};

const isPaused = (): boolean => animationId === null;

const play = (): void => {
  playPauseIcon.src = "/static/pause.svg";
  playPauseIcon.alt = "Pause";
  renderLoop();
};

const pause = (): void => {
  if (animationId !== null) {
    playPauseIcon.src = "/static/play.svg";
    playPauseIcon.alt = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
  }
};

document.getElementById("play-pause-button")!.addEventListener("click", () => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

const resetState = (): void => {
  const newState = initialStates[mode];
  canvas = newState.canvas;

  if (!lockParameters) {
    width = newState.width;
    height = newState.height;
    numSources = newState.numSources;
    movementSpeed = newState.movementSpeed;
    colorSpeed = newState.colorSpeed;
  }

  restartSpectrum();
};

modeWebgl.addEventListener("click", () => {
  if (mode !== "webgl") {
    (mode === "wasm" ? modeWasm : modeJs).classList.remove("current-mode");
    modeWebgl.classList.add("current-mode");
    mode = "webgl";
    resetState();
  }
});

modeWasm.addEventListener("click", () => {
  if (mode !== "wasm") {
    (mode === "webgl" ? modeWebgl : modeJs).classList.remove("current-mode");
    modeWasm.classList.add("current-mode");
    mode = "wasm";
    resetState();
  }
});

modeJs.addEventListener("click", () => {
  if (mode !== "js") {
    (mode === "webgl" ? modeWebgl : modeWasm).classList.remove("current-mode");
    modeJs.classList.add("current-mode");
    mode = "js";
    resetState();
  }
});

modeUnlock.addEventListener("click", () => {
  if (lockParameters) {
    modeLock.classList.remove("current-mode");
    modeUnlock.classList.add("current-mode");
    lockParameters = false;
  }
});

modeLock.addEventListener("click", () => {
  if (!lockParameters) {
    modeUnlock.classList.remove("current-mode");
    modeLock.classList.add("current-mode");
    lockParameters = true;
  }
});

setWidth.max = MAX_WIDTH.toString();
setWidth.value = width.toString();
setWidth.addEventListener("change", (e) => {
  const newWidth = (e.target as HTMLInputElement).value;
  width = parseInt(newWidth);
  widthText.textContent = width.toString();
  restartSpectrum();
});

setHeight.max = MAX_HEIGHT.toString();
setHeight.value = height.toString();
setHeight.addEventListener("change", (e) => {
  const newHeight = (e.target as HTMLInputElement).value;
  height = parseInt(newHeight);
  heightText.textContent = height.toString();
  restartSpectrum();
});

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

setMovementSpeed.value = movementSpeed.toString();
setMovementSpeed.addEventListener("change", (e) => {
  const newMovementSpeed = (e.target as HTMLInputElement).value;
  movementSpeed = parseInt(newMovementSpeed);
  movementSpeedText.textContent = movementSpeed.toString();
  restartSpectrum();
});

setColorSpeed.value = colorSpeed.toString();
setColorSpeed.addEventListener("change", (e) => {
  const newColorSpeed = (e.target as HTMLInputElement).value;
  colorSpeed = parseInt(newColorSpeed);
  colorSpeedText.textContent = colorSpeed.toString();
  restartSpectrum();
});

document.getElementById("restart-button")!.addEventListener("click", () => {
  spectrum = getNewSpectrum();
});

document.getElementById("download-link")!.addEventListener("click", () => {
  downloadLink.setAttribute(
    "href",
    canvas.toDataURL("image/png").replace("image/png", "image/octet-stream")
  );
});

collapse.addEventListener("click", () => {
  controls.classList.add("hide-controls");
  setTimeout(() => expand.classList.remove("hide-expand"), 500);
});

expand.addEventListener("click", () => {
  expand.classList.add("hide-expand");
  controls.classList.remove("hide-controls");
});

controls.classList.remove("hide");
play();
