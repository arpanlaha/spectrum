/* eslint-disable @typescript-eslint/no-magic-numbers */

// WebAssembly support check derived from https://stackoverflow.com/a/47880734/13172180
let supported = true;
try {
  if (
    typeof WebAssembly === "object" &&
    typeof WebAssembly.instantiate === "function"
  ) {
    const module = new WebAssembly.Module(
      Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
    );
    if (module instanceof WebAssembly.Module) {
      supported =
        new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
    }
  } else {
    supported = false;
  }
} catch (e) {
  supported = false;
}

if (supported) {
  console.log("hello");
  import("./index").catch((e) => {
    document.getElementById(
      "error-message"
    )!.textContent = `An error was encountered: ${e}`;
    document.getElementById("no-wasm")!.classList.remove("hide");
  });
} else {
  document.getElementById("canvas-webgl")!.classList.add("hide");
  document.getElementById("no-wasm")!.classList.remove("hide");
}
