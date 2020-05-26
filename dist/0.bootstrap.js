(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var hello_wasm_pack__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! hello-wasm-pack */ \"./node_modules/hello-wasm-pack/hello_wasm_pack.js\");\n\n\nhello_wasm_pack__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/hello-wasm-pack/hello_wasm_pack.js":
/*!*********************************************************!*\
  !*** ./node_modules/hello-wasm-pack/hello_wasm_pack.js ***!
  \*********************************************************/
/*! exports provided: __wbg_alert_955be295a438967b, greet */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_955be295a438967b\", function() { return __wbg_alert_955be295a438967b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony import */ var _hello_wasm_pack_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./hello_wasm_pack_bg */ \"./node_modules/hello-wasm-pack/hello_wasm_pack_bg.wasm\");\n/* tslint:disable */\n\n\nlet cachedDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _hello_wasm_pack_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_hello_wasm_pack_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_alert_955be295a438967b(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    alert(varg0);\n}\n/**\n* @returns {void}\n*/\nfunction greet() {\n    return _hello_wasm_pack_bg__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n}\n\n\n\n//# sourceURL=webpack:///./node_modules/hello-wasm-pack/hello_wasm_pack.js?");

/***/ }),

/***/ "./node_modules/hello-wasm-pack/hello_wasm_pack_bg.wasm":
/*!**************************************************************!*\
  !*** ./node_modules/hello-wasm-pack/hello_wasm_pack_bg.wasm ***!
  \**************************************************************/
/*! exports provided: memory, __indirect_function_table, __heap_base, __data_end, greet */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./hello_wasm_pack */ \"./node_modules/hello-wasm-pack/hello_wasm_pack.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./node_modules/hello-wasm-pack/hello_wasm_pack_bg.wasm?");

/***/ })

}]);