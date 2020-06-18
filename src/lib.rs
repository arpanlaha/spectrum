mod spectrum;
mod utils;

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement, HtmlImageElement,
    HtmlInputElement, WebGlRenderingContext,
};

use spectrum::wasm::SpectrumWasm;
use spectrum::webgl::SpectrumGL;

const WEBGL_SCALE: f32 = 1f32;
const WASM_SCALE: f32 = 0.4f32;
const JS_SCALE: f32 = 0.25f32;
const MOVEMENT_SPEED_FACTOR: f32 = 0.2f32;
const COLOR_SPEED_FACTOR: f32 = 0.002f32;
const UNIFORMS_PER_SOURCE: usize = 4;

struct State {
    pub canvas: HtmlCanvasElement,
    pub width: usize,
    pub height: usize,
    pub num_sources: usize,
    pub movement_speed: f32,
    pub color_speed: f32,
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas_webgl = document
        .get_element_by_id("canvas-webgl")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let canvas_2d = document
        .get_element_by_id("canvas-wasm")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let controls = document.get_element_by_id("controls").unwrap();

    let play_pause_icon = document
        .get_element_by_id("play-pause-icon")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    let download_link = document
        .get_element_by_id("download-link")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();

    let mode_webgl = document.get_element_by_id("mode-webgl").unwrap();
    let mode_wasm = document.get_element_by_id("mode-wasm").unwrap();
    let mode_js = document.get_element_by_id("mode-js").unwrap();
    let mode_lock = document.get_element_by_id("mode-lock").unwrap();
    let mode_unlock = document.get_element_by_id("mode-unlock").unwrap();

    let width_text = document.get_element_by_id("width").unwrap();
    let set_width = document
        .get_element_by_id("set-width")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let height_text = document.get_element_by_id("height").unwrap();
    let set_height = document
        .get_element_by_id("set-height")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let num_sources_text = document.get_element_by_id("num-sources").unwrap();
    let set_num_sources = document
        .get_element_by_id("set-num_sources")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let movement_speed_text = document.get_element_by_id("movement-speed").unwrap();
    let set_movement_speed = document
        .get_element_by_id("set-movement_speed")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let color_speed_text = document.get_element_by_id("color-speed").unwrap();
    let set_color_speed = document
        .get_element_by_id("set-color")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let collapse = document
        .get_element_by_id("collapse")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();
    let expand = document
        .get_element_by_id("expand")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    let context_webgl_options = Object::new();
    Reflect::set(
        &context_webgl_options,
        &"preserveDrawingBuffer".into(),
        &wasm_bindgen::JsValue::TRUE,
    )
    .unwrap();

    let context_webgl = canvas_webgl
        .get_context_with_context_options("webgl", &context_webgl_options)
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    let context_2d = canvas_2d
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context_2d
        .scale(window.device_pixel_ratio(), window.device_pixel_ratio())
        .unwrap();
}
