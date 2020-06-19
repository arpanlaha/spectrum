mod spectrum;
mod utils;

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, Event, HtmlAnchorElement, HtmlCanvasElement, HtmlImageElement,
    HtmlInputElement, WebGlRenderingContext, Window,
};

// use spectrum::wasm::SpectrumWasm;
// use spectrum::webgl::SpectrumGL;

const WEBGL_SCALE: f32 = 1f32;
const WASM_SCALE: f32 = 0.4f32;
const JS_SCALE: f32 = 0.25f32;
const MOVEMENT_SPEED_FACTOR: f32 = 0.2f32;
const COLOR_SPEED_FACTOR: f32 = 0.002f32;
const UNIFORMS_PER_SOURCE: usize = 4;
const MIN_DIMENSION: &str = "100";

struct State {
    pub canvas: HtmlCanvasElement,
    pub width: usize,
    pub height: usize,
    pub num_sources: usize,
    pub movement_speed: f32,
    pub color_speed: f32,
}

fn init_input(param: &str, min: &str, max: &str, step: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let setter_id = format!("set-{}", param);
    let text = document.get_element_by_id(param).unwrap();
    let setter = document
        .get_element_by_id(&setter_id[..])
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    setter.set_min(min);
    setter.set_max(max);
    setter.set_step(step);

    let onchange = Closure::wrap(Box::new(move || {
        let value = &web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&setter_id[..])
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()[..];

        text.set_text_content(Some(value));
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("param", value)
            .unwrap();
    }) as Box<dyn Fn()>);

    setter.set_onchange(Some(onchange.as_ref().unchecked_ref()));

    onchange.forget();

    // setter.set_onchang
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

    init_input(
        "width",
        MIN_DIMENSION,
        &(window.device_pixel_ratio() * window.inner_width().unwrap().as_f64().unwrap())
            .to_string()[..],
        "10",
    );

    init_input(
        "height",
        MIN_DIMENSION,
        &(window.device_pixel_ratio() * window.inner_height().unwrap().as_f64().unwrap())
            .to_string()[..],
        "10",
    );

    init_input("num-sources", "2", "100", "1");
    init_input("movement-speed", "1", "100", "1");
    init_input("color-speed", "1", "100", "1");

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

    // set_width.set_onchange(Some(
    //     Closure::wrap(Box::new(|| {
    //         web_sys::window()
    //             .unwrap()
    //             .local_storage()
    //             .unwrap()
    //             .unwrap()
    //             .set_item(
    //                 "width",
    //                 &web_sys::window()
    //                     .unwrap()
    //                     .document()
    //                     .unwrap()
    //                     .get_element_by_id("set-width")
    //                     .unwrap()
    //                     .dyn_into::<HtmlInputElement>()
    //                     .unwrap()
    //                     .value(),
    //             )
    //             .unwrap();
    //     }) as Box<dyn Fn()>)
    //     .as_ref()
    //     .unchecked_ref(),
    // ));
}
