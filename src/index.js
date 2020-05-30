import { Spectrum } from "wasm-spectrum";
import { memory } from "wasm-spectrum/spectrum_bg";

const BYTES_PER_PIXEL = 4;

const PAGE_FRACTION = 3;

const scale = window.devicePixelRatio;

let { clientWidth, clientHeight } = document.body;

clientWidth = Math.round(clientWidth / PAGE_FRACTION);
clientHeight = Math.round(clientHeight / PAGE_FRACTION);

const width = Math.round(clientWidth * scale);
const height = Math.round(clientHeight * scale);
const numSources = Math.round(
  Math.sqrt(Math.sqrt(width * height)) / PAGE_FRACTION
);

const canvas = document.getElementById("spectrum-canvas");
const context = canvas.getContext("2d");

canvas.style.width = `${clientWidth}px`;
canvas.style.height = `${clientHeight}px`;
canvas.width = width;
canvas.height = height;
context.scale(scale, scale);

const spectrum = Spectrum.new(width, height, numSources);

// for (let i = 0; i < 5; i++) {
//   console.time(`${i}`);

//   const spectrumData = spectrum.data();
//   const spectrumArray = new Uint8ClampedArray(
//     memory.buffer,
//     spectrumData,
//     width * height * BYTES_PER_PIXEL
//   );
//   const spectrumImageData = new ImageData(spectrumArray, width, height);

//   context.putImageData(spectrumImageData, 0, 0);

//   spectrum.tick();
//   spectrum.draw();
//   console.timeEnd(`${i}`);
// }
// canvas.classList.add("show");

const renderLoop = () => {
  // console.time("hi");

  const spectrumData = spectrum.data();
  const spectrumArray = new Uint8ClampedArray(
    memory.buffer,
    spectrumData,
    width * height * BYTES_PER_PIXEL
  );
  const spectrumImageData = new ImageData(spectrumArray, width, height);

  context.putImageData(spectrumImageData, 0, 0);

  spectrum.tick();
  spectrum.draw();
  // console.timeEnd("hi");

  window.requestAnimationFrame(renderLoop);
};

window.requestAnimationFrame(renderLoop);
