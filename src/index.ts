import { SpectrumWebGL, SpectrumWasm } from "wasm-spectrum";
import { SpectrumJS } from "./spectrum";
import { FPS } from "./utils";

// Constants
const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;
const WEBGL_SCALE = 1;
const WASM_SCALE = 0.4;
const JS_SCALE = 0.25;
const UNIFORMS_PER_SOURCE = 4;

// Types

type Mode = "webgl" | "wasm" | "js";

type Param =
  | "width"
  | "height"
  | "numSources"
  | "movementSpeed"
  | "colorSpeed"
  | "sourceDropoff";

interface Spectrum {
  draw: () => void;
  tick: () => void;
}

interface InitialState {
  canvas: HTMLCanvasElement;
  width: number;
  height: number;
  numSources: number;
  movementSpeed: number;
  colorSpeed: number;
  sourceDropoff: number;
}

interface State extends InitialState {
  spectrum: Spectrum;
}

const modes: Mode[] = ["webgl", "wasm", "js"];

const params: Param[] = [
  "width",
  "height",
  "numSources",
  "movementSpeed",
  "colorSpeed",
  "sourceDropoff",
];

/**
 * Converts parameters from camelCase (for use in scripts) to kebab-case (for use in HTML/CSS).
 */
const kebabParams: Record<Param, string> = {
  width: "width",
  height: "height",
  numSources: "num-sources",
  movementSpeed: "movement-speed",
  colorSpeed: "color-speed",
  sourceDropoff: "source-dropoff",
};

// Reused html elements
const canvasWebgl = document.getElementById(
  "canvas-webgl"
) as HTMLCanvasElement;
const canvas2d = document.getElementById("canvas-2d") as HTMLCanvasElement;
const controls = document.getElementById("controls") as HTMLDivElement;
const playPauseIcon = document.getElementById(
  "play-pause-icon"
) as HTMLImageElement;
const modeLock = document.getElementById("mode-lock") as HTMLDivElement;
const modeUnlock = document.getElementById("mode-unlock") as HTMLDivElement;
const expand = document.getElementById("expand") as HTMLImageElement;

const contextWebgl = canvasWebgl.getContext("webgl", {
  powerPreference: "high-performance",
  preserveDrawingBuffer: true,
}) as WebGLRenderingContext;

// Scale canvas to account for device pixel ratio.
(canvas2d.getContext("2d") as CanvasRenderingContext2D).scale(
  DEVICE_SCALE,
  DEVICE_SCALE
);

/**
 * Max hardware-supported number of Sources for WebGL.
 *
 * iOS is dumb and has a limited number of fragment shader uniforms.
 */
const WEBGL_NUM_SOURCES_UPPER_BOUND = Math.floor(
  contextWebgl.getParameter(contextWebgl.MAX_FRAGMENT_UNIFORM_VECTORS) /
    UNIFORMS_PER_SOURCE
);

/**
 * Hardware-enforced upper limits to parameters.
 */
const UPPER_BOUNDS = {
  height: MAX_HEIGHT,
  width: MAX_WIDTH,
  numSources: Math.min(100, WEBGL_NUM_SOURCES_UPPER_BOUND),
};

/* eslint-disable @typescript-eslint/no-magic-numbers */
/**
 * Default parameters for different modes.
 */
const modeStates: Record<Mode, InitialState> = {
  webgl: {
    canvas: canvasWebgl,
    width: Math.round(MAX_WIDTH * WEBGL_SCALE),
    height: Math.round(MAX_HEIGHT * WEBGL_SCALE),
    numSources: Math.min(20, WEBGL_NUM_SOURCES_UPPER_BOUND),
    movementSpeed: 10,
    colorSpeed: 10,
    sourceDropoff: 50,
  },
  wasm: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * WASM_SCALE),
    height: Math.round(MAX_HEIGHT * WASM_SCALE),
    numSources: 10,
    movementSpeed: 10,
    colorSpeed: 20,
    sourceDropoff: 50,
  },
  js: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * JS_SCALE),
    height: Math.round(MAX_HEIGHT * JS_SCALE),
    numSources: 10,
    movementSpeed: 10,
    colorSpeed: 20,
    sourceDropoff: 50,
  },
};
/* eslint-enable @typescript-eslint/no-magic-numbers */

/**
 * Contains each mode's associated implementation.
 */
const spectrumInitializers = {
  webgl: SpectrumWebGL,
  wasm: SpectrumWasm,
  js: SpectrumJS,
};

/**
 * Resets parameters and associated inputs, as well as resizing the current canvas.
 * @param state the State passed in to reset parameters to.
 */
const resetParams = (state: InitialState): void => {
  const { canvas, width, height } = state;

  params.forEach((param) => {
    document.getElementById(kebabParams[param])!.textContent =
      state[param].toString();
    (
      document.getElementById(`set-${kebabParams[param]}`) as HTMLInputElement
    ).value = state[param].toString();
  });

  canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
  canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }
};

/**
 * Constructs the initial State.
 * @param mode the initial mode.
 */
const getInitialState = (mode: Mode): State => {
  const {
    width,
    height,
    numSources,
    movementSpeed,
    colorSpeed,
    sourceDropoff,
  } = modeStates[mode];

  resetParams(modeStates[mode]);

  const spectrum = spectrumInitializers[mode].new(
    width,
    height,
    numSources,
    movementSpeed,
    colorSpeed,
    sourceDropoff,
    mode === "webgl" ? canvasWebgl : canvas2d
  );

  return {
    spectrum,
    ...modeStates[mode],
  };
};

/**
 * The rendering mode.
 */
let mode: Mode = "webgl";

/**
 * Whether to lock parameters or use defaults on a mode change.
 */
let lockParameters = false;

/**
 * The current application state.
 */
let state = getInitialState(mode);

/**
 * The current requested animation frame id if playing, or null if not.
 */
let animationId: number | null = null;

/**
 * Fetches a new Spectrum based on current state.
 */
const getNewSpectrum = (): Spectrum => {
  const {
    width,
    height,
    numSources,
    movementSpeed,
    colorSpeed,
    sourceDropoff,
  } = state;
  resetParams(state);
  return spectrumInitializers[mode].new(
    width,
    height,
    numSources,
    movementSpeed,
    colorSpeed,
    sourceDropoff,
    mode === "webgl" ? canvasWebgl : canvas2d
  );
};

/**
 * The current FPS object rendering to the corresponding HTML element.
 */
let fps = new FPS();

/**
 * Restarts the Spectrum.
 */
const restartSpectrum = (): void => {
  const shouldPlay = animationId !== null;

  if (shouldPlay) {
    pause();
  }

  fps = new FPS();

  (mode === "webgl" ? canvas2d : canvasWebgl).classList.add("hide");
  (mode === "webgl" ? canvasWebgl : canvas2d).classList.remove("hide");

  state.spectrum = getNewSpectrum();

  if (shouldPlay) {
    play();
  }
};

/**
 * Called on each frame, drawing the Spectrum, updating FPS, and updating Spectrum state.
 */
const renderLoop = (): void => {
  const { spectrum } = state;
  spectrum.draw();
  fps.render();
  spectrum.tick();

  animationId = window.requestAnimationFrame(renderLoop);
};

/**
 * Plays the Spectrum.
 */
const play = (): void => {
  playPauseIcon.src = "/static/pause.svg";
  playPauseIcon.alt = "Pause";
  animationId = window.requestAnimationFrame(renderLoop);
};

/**
 * Pauses the Spectrum.
 */
const pause = (): void => {
  if (animationId !== null) {
    playPauseIcon.src = "/static/play.svg";
    playPauseIcon.alt = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
  }
};

// Play or pause depending on application state.
document.getElementById("play-pause-button")!.addEventListener("click", () => {
  if (animationId === null) {
    play();
  } else {
    pause();
  }
});

// Set up restart button event listener.
document
  .getElementById("restart-button")!
  .addEventListener("click", () => (state.spectrum = getNewSpectrum()));

// Set up download button event listener.
(
  document.getElementById("download-link") as HTMLAnchorElement
).addEventListener("click", function () {
  this.href = state.canvas
    .toDataURL("image/png")
    .replace("image/png", "image/octet-stream");
});

// Set up event listeners for each mode button.
modes.forEach((modeName) => {
  const modeButton = document.getElementById(
    `mode-${modeName}`
  ) as HTMLDivElement;
  modeButton.addEventListener("click", () => {
    if (mode !== modeName) {
      document.getElementById(`mode-${mode}`)!.classList.remove("current-mode");
      modeButton.classList.add("current-mode");
      mode = modeName;
      const newState = modeStates[mode];
      state.canvas = newState.canvas;
      if (!lockParameters) {
        state = { ...state, ...newState };
      }
      restartSpectrum();
    }
  });
});

// Set up unlock event listeners.
modeUnlock.addEventListener("click", () => {
  if (lockParameters) {
    modeLock.classList.remove("current-mode");
    modeUnlock.classList.add("current-mode");
    lockParameters = false;
  }
});

// Set up lock event listeners.
modeLock.addEventListener("click", () => {
  if (!lockParameters) {
    modeUnlock.classList.remove("current-mode");
    modeLock.classList.add("current-mode");
    lockParameters = true;
  }
});

// Set up parameter input event listeners.
params.forEach((param) => {
  const setter = document.getElementById(
    `set-${kebabParams[param]}`
  )! as HTMLInputElement;

  if (param === "width" || param === "height" || param === "numSources") {
    setter.max = UPPER_BOUNDS[param].toString();
  }

  setter.value = state[param].toString();
  setter.addEventListener("change", (e) => {
    const newParam = (e.target as HTMLInputElement).value;
    state[param] = parseInt(newParam);
    document.getElementById(kebabParams[param])!.textContent =
      newParam.toString();
    restartSpectrum();
  });
});

// Set up controls collapse event listener.
document.getElementById("collapse")!.addEventListener("click", () => {
  controls.classList.add("hide-controls");
  setTimeout(() => expand.classList.remove("hide-expand"), 500);
});

// Set up controls expand event listener.
expand.addEventListener("click", () => {
  expand.classList.add("hide-expand");
  controls.classList.remove("hide-controls");
});

// Show hidden objects after setup is complete.
state.canvas.classList.remove("hide");
controls.classList.remove("hide");

// Start the Spectrum.
play();
