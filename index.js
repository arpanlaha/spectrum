import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");
const width = document.body.clientWidth;
const height = document.body.clientHeight;
canvas.width = width;
canvas.height = height;
const numSources = Math.round(Math.sqrt(Math.sqrt(width * height)));
const spectrum = Spectrum.new(width, height, numSources);
const spectrumData = spectrum.data();
const spectrumArray = new Uint8ClampedArray(
  memory.buffer,
  spectrumData,
  width * height * 4
);
const spectrumImageData = new ImageData(spectrumArray, width, height);
context.putImageData(spectrumImageData, 0, 0);
