import { SpectrumWebGL, SpectrumWasm } from "wasm-spectrum";
import { SpectrumJS } from "./spectrum";
import { FPS } from "./utils";

const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;
const WEBGL_SCALE = 1;
const WASM_SCALE = 0.4;
const JS_SCALE = 0.25;
const MOVEMENT_SPEED_FACTOR = 0.2;
const COLOR_SPEED_FACTOR = 0.002;
const UNIFORMS_PER_SOURCE = 4;

type Mode = "webgl" | "wasm" | "js";

type Param = "width" | "height" | "numSources" | "movementSpeed" | "colorSpeed";

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
];

const kebabParams = {
  width: "width",
  height: "height",
  numSources: "num-sources",
  movementSpeed: "movement-speed",
  colorSpeed: "color-speed",
};

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
  preserveDrawingBuffer: true,
}) as WebGLRenderingContext;

(canvas2d.getContext("2d") as CanvasRenderingContext2D).scale(
  DEVICE_SCALE,
  DEVICE_SCALE
);

const WEBGL_NUM_SOURCES_UPPER_BOUND = Math.floor(
  contextWebgl.getParameter(contextWebgl.MAX_FRAGMENT_UNIFORM_VECTORS) /
    UNIFORMS_PER_SOURCE
);

const UPPER_BOUNDS = {
  height: MAX_HEIGHT,
  width: MAX_WIDTH,
  numSources: WEBGL_NUM_SOURCES_UPPER_BOUND,
};

/* eslint-disable @typescript-eslint/no-magic-numbers */
const modeStates: Record<Mode, InitialState> = {
  webgl: {
    canvas: canvasWebgl,
    width: Math.round(MAX_WIDTH * WEBGL_SCALE),
    height: Math.round(MAX_HEIGHT * WEBGL_SCALE),
    numSources: Math.min(
      20,
      // iOS is dumb and has a limited number of fragment shader uniforms
      WEBGL_NUM_SOURCES_UPPER_BOUND
    ),
    movementSpeed: 10,
    colorSpeed: 10,
  },
  wasm: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * WASM_SCALE),
    height: Math.round(MAX_HEIGHT * WASM_SCALE),
    numSources: 10,
    movementSpeed: 10,
    colorSpeed: 20,
  },
  js: {
    canvas: canvas2d,
    width: Math.round(MAX_WIDTH * JS_SCALE),
    height: Math.round(MAX_HEIGHT * JS_SCALE),
    numSources: 10,
    movementSpeed: 10,
    colorSpeed: 20,
  },
};
/* eslint-enable @typescript-eslint/no-magic-numbers */

const spectrumInitializers = {
  webgl: SpectrumWebGL,
  wasm: SpectrumWasm,
  js: SpectrumJS,
};

const resetParams = (state: InitialState): void => {
  const { canvas, width, height } = state;

  params.forEach((param) => {
    document.getElementById(kebabParams[param])!.textContent = state[
      param
    ].toString();
    (document.getElementById(
      `set-${kebabParams[param]}`
    ) as HTMLInputElement).value = state[param].toString();
  });

  canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
  canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
  canvas.width = width;
  canvas.height = height;

  if (mode === "webgl") {
    contextWebgl.viewport(0, 0, width, height);
  }
};

const getInitialState = (mode: Mode): State => {
  const { width, height, numSources, movementSpeed, colorSpeed } = modeStates[
    mode
  ];

  resetParams(modeStates[mode]);

  const spectrum = spectrumInitializers[mode].new(
    width,
    height,
    numSources,
    movementSpeed * MOVEMENT_SPEED_FACTOR,
    colorSpeed * COLOR_SPEED_FACTOR,
    mode === "webgl" ? canvasWebgl : canvas2d
  );

  return {
    spectrum,
    ...modeStates[mode],
  };
};

let mode: Mode = "webgl";
let lockParameters = false;
let state = getInitialState(mode);
let animationId: number | null = null;

const getNewSpectrum = (): Spectrum => {
  const { width, height, numSources, movementSpeed, colorSpeed } = state;
  resetParams(state);
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

const renderLoop = (): void => {
  const { spectrum } = state;
  spectrum.draw();
  fps.render();
  spectrum.tick();

  animationId = window.requestAnimationFrame(renderLoop);
};

const play = (): void => {
  playPauseIcon.src = "/static/pause.svg";
  playPauseIcon.alt = "Pause";
  animationId = window.requestAnimationFrame(renderLoop);
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
  if (animationId === null) {
    play();
  } else {
    pause();
  }
});

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
    document.getElementById(
      kebabParams[param]
    )!.textContent = newParam.toString();
    restartSpectrum();
  });
});

document
  .getElementById("restart-button")!
  .addEventListener("click", () => (state.spectrum = getNewSpectrum()));

(document.getElementById(
  "download-link"
) as HTMLAnchorElement).addEventListener("click", function () {
  this.href = state.canvas
    .toDataURL("image/png")
    .replace("image/png", "image/octet-stream");
});

document.getElementById("collapse")!.addEventListener("click", () => {
  controls.classList.add("hide-controls");
  setTimeout(() => expand.classList.remove("hide-expand"), 500);
});

expand.addEventListener("click", () => {
  expand.classList.add("hide-expand");
  controls.classList.remove("hide-controls");
});

state.canvas.classList.remove("hide");
controls.classList.remove("hide");
play();
