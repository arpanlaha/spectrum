/* eslint-disable @typescript-eslint/no-magic-numbers */

// WebAssembly support check derived from https://stackoverflow.com/a/47880734/13172180
let supportsWasm = true;
try {
  let module: WebAssembly.Module | null = null;
  supportsWasm =
    typeof WebAssembly === "object" &&
    typeof WebAssembly.instantiate === "function" &&
    (module = new WebAssembly.Module(
      Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
    )) instanceof WebAssembly.Module &&
    new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
} catch (e) {
  supportsWasm = false;
}

// dynamic import support check derived from https://stackoverflow.com/a/60317331/13172180
let supportsDynamicImport = true;
try {
  eval("try { import('foo').catch(() => {}); } catch (e) { }");
} catch (e) {
  supportsDynamicImport = false;
}

// run app logic if support detected
if (supportsWasm && supportsDynamicImport) {
  import("./index").catch((e) =>
    console.error("Error importing `index.js`:", e)
  );
} else {
  // otherwise display error messages with missing browser support information
  if (!supportsWasm) {
    document.getElementById("no-wasm")!.classList.remove("hide");
  }
  if (!supportsDynamicImport) {
    document.getElementById("no-dynamic-import")!.classList.remove("hide");
  }
}
