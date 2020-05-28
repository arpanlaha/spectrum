import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

console.log("Hello");

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");
// const width = Math.round(document.body.clientWidth / 3);
const width = document.body.clientWidth;
const height = document.body.clientHeight;
// console.log(width, height);
const numSources = Math.round(Math.sqrt(width * height));
const spectrum = Spectrum.new(width, height, numSources);
const spectrumData = spectrum.data();
// console.log(spectrumData);
const spectrumArray = new Uint8ClampedArray(
  memory.buffer,
  spectrumData,
  width * height * 4
);

const spectrumImageData = new ImageData(spectrumArray, width, height);
const index = 100000;
// ["r", "g", "b", "a"].forEach((channel, idx) =>
//   console.log(`${channel}: ${spectrumArray[4 * index + idx]}`)
// );
context.putImageData(spectrumImageData, 0, 0);

console.log("goodbye");
