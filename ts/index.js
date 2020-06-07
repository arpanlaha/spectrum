import { Spectrum, SpectrumGL } from "wasm-spectrum";
import FPS from "./utils/fps";
const DEVICE_SCALE = window.devicePixelRatio;
const MAX_WIDTH = document.body.clientWidth * DEVICE_SCALE;
const MAX_HEIGHT = document.body.clientHeight * DEVICE_SCALE;
const WASM_SCALE = 0.4;
const GL_SCALE = 0.8;
const MODE_LABELS = {
    wasm: "WebAssembly",
    webgl: "WebGL",
};
const canvasWebgl = document.getElementById("canvas-webgl");
const canvasWasm = document.getElementById("canvas-wasm");
const controls = document.getElementById("controls");
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
const initialStates = {
    webgl: {
        canvas: canvasWebgl,
        width: Math.round(MAX_WIDTH * GL_SCALE),
        height: Math.round(MAX_HEIGHT * GL_SCALE),
        numSources: 20,
    },
    wasm: {
        canvas: canvasWasm,
        width: Math.round(MAX_WIDTH * WASM_SCALE),
        height: Math.round(MAX_HEIGHT * WASM_SCALE),
        numSources: 10,
    },
};
const getInitialState = (mode) => {
    const { canvas, width, height, numSources } = initialStates[mode];
    widthText.textContent = width.toString();
    heightText.textContent = height.toString();
    numSourcesText.textContent = numSources.toString();
    canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
    canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
    canvas.width = width;
    canvas.height = height;
    if (mode === "webgl") {
        contextWebgl.viewport(0, 0, width, height);
    }
    const spectrum = mode === "webgl"
        ? SpectrumGL.new(width, height, numSources, contextWebgl)
        : Spectrum.new(width, height, numSources, contextWasm);
    return { canvas, width, height, numSources, spectrum };
};
let mode = "webgl";
let { canvas, width, height, numSources, spectrum } = getInitialState(mode);
let animationId = null;
const setupCanvas = () => {
    widthText.textContent = width.toString();
    heightText.textContent = height.toString();
    numSourcesText.textContent = numSources.toString();
    canvas.style.width = `${Math.round(width / DEVICE_SCALE)}px`;
    canvas.style.height = `${Math.round(height / DEVICE_SCALE)}px`;
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
let fps = new FPS();
const restartSpectrum = () => {
    const shouldPlay = !isPaused();
    if (shouldPlay) {
        pause();
    }
    fps = new FPS();
    if (mode === "webgl") {
        canvasWasm.classList.add("hide");
        canvasWebgl.classList.remove("hide");
        canvas = canvasWebgl;
        spectrum = getNewSpectrumGl();
    }
    else if (mode === "wasm") {
        canvasWebgl.classList.add("hide");
        canvasWasm.classList.remove("hide");
        canvas = canvasWasm;
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
    const newWidth = e.target.value;
    width = parseInt(newWidth);
    widthText.textContent = width.toString();
    restartSpectrum();
});
setHeight.min = "100";
setHeight.max = MAX_HEIGHT.toString();
setHeight.value = height.toString();
setHeight.addEventListener("change", (e) => {
    const newHeight = e.target.value;
    height = parseInt(newHeight);
    heightText.textContent = height.toString();
    restartSpectrum();
});
setNumSources.min = "2";
setNumSources.max = "100";
setNumSources.value = numSources.toString();
setNumSources.addEventListener("change", (e) => {
    const newNumSources = e.target.value;
    numSources = parseInt(newNumSources);
    numSourcesText.textContent = numSources.toString();
    restartSpectrum();
});
const drawFrame = () => {
    spectrum.draw();
    fps.render();
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
    if (animationId !== null) {
        playPauseButton.textContent = "▶";
        cancelAnimationFrame(animationId);
        animationId = null;
    }
};
playPauseButton.addEventListener("click", () => {
    if (isPaused()) {
        play();
    }
    else {
        pause();
    }
});
toggleButton.addEventListener("click", () => {
    if (mode === "webgl") {
        mode = "wasm";
    }
    else if (mode === "wasm") {
        mode = "webgl";
    }
    modeText.textContent = MODE_LABELS[mode];
    restartSpectrum();
});
controls.classList.remove("hide");
play();
