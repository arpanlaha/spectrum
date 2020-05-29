import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const BYTES_PER_PIXEL = 4;

const width = document.body.clientWidth;
const height = document.body.clientHeight;
const numSources = Math.round(Math.sqrt(Math.sqrt(width * height)));

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");
canvas.width = width;
canvas.height = height;

const spectrum = Spectrum.new(width, height, numSources);
const spectrumData = spectrum.data();
const spectrumArray = new Uint8ClampedArray(
  memory.buffer,
  spectrumData,
  width * height * BYTES_PER_PIXEL
);
const spectrumImageData = new ImageData(spectrumArray, width, height);

context.putImageData(spectrumImageData, 0, 0);
