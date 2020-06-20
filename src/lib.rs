mod spectrum;
mod utils;

use std::cmp;
use std::fmt::Display;

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement, HtmlElement, HtmlImageElement,
    HtmlInputElement, Storage, WebGlRenderingContext,
};

use spectrum::wasm::SpectrumWasm;
use spectrum::webgl::SpectrumGL;
use utils::base::Spectrum;

const WEBGL_SCALE: f64 = 1f64;
const WASM_SCALE: f64 = 0.4f64;
// const JS_SCALE: f64 = 0.25f64;
const MOVEMENT_SPEED_FACTOR: f32 = 0.2f32;
const COLOR_SPEED_FACTOR: f32 = 0.002f32;
const MIN_DIMENSION: &str = "100";

static mut SPECTRUM: Option<Box<dyn Spectrum>> = None;
static mut FRAME: Option<i32> = None;

struct State {
    pub width: usize,
    pub height: usize,
    pub num_sources: usize,
    pub movement_speed: f32,
    pub color_speed: f32,
}

fn max_width() -> f64 {
    let window = web_sys::window().unwrap();
    (window.device_pixel_ratio() * window.inner_width().unwrap().as_f64().unwrap()).round()
}

fn max_height() -> f64 {
    let window = web_sys::window().unwrap();
    (window.device_pixel_ratio() * window.inner_height().unwrap().as_f64().unwrap()).round()
}

fn get_local_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

fn get_local_storage_item(key: &str) -> String {
    get_local_storage().get_item(key).unwrap().unwrap()
}

fn set_local_storage_item(key: &str, value: &str) {
    get_local_storage().set_item(key, value).unwrap();
}

fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(if get_local_storage_item("mode") == "webgl" {
            "canvas-webgl"
        } else {
            "canvas-wasm"
        })
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}

fn get_default_params() -> State {
    if get_local_storage_item("mode") == "webgl" {
        let context_webgl_options = Object::new();
        Reflect::set(
            &context_webgl_options,
            &"preserveDrawingBuffer".into(),
            &wasm_bindgen::JsValue::TRUE,
        )
        .unwrap();

        let context_webgl = get_canvas()
            .get_context_with_context_options("webgl", &context_webgl_options)
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        State {
            width: (max_width() * WEBGL_SCALE) as usize,
            height: (max_height() * WEBGL_SCALE) as usize,
            num_sources: cmp::min(
                20,
                context_webgl
                    .get_parameter(WebGlRenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS)
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize
                    / 4,
            ),
            movement_speed: 10f32,
            color_speed: 10f32,
        }
    } else {
        State {
            width: (max_width() * WASM_SCALE) as usize,
            height: (max_height() * WASM_SCALE) as usize,
            num_sources: 10,
            movement_speed: 10f32,
            color_speed: 20f32,
        }
    }
}

fn get_or_set_initial_value<T: Display>(key: &str, default: T) -> String {
    let local_storage = get_local_storage();
    let default_str = default.to_string();

    match local_storage.get_item(key).unwrap() {
        Some(value) => value,
        None => {
            local_storage.set_item(key, &default_str[..]).unwrap();
            default_str
        }
    }
}

fn resize_canvas() {
    let canvas = get_canvas();

    let width = get_or_set_initial_value("width", &max_width().to_string()[..])
        .parse::<u32>()
        .unwrap();

    let height = get_or_set_initial_value("height", &max_height().to_string()[..])
        .parse::<u32>()
        .unwrap();

    canvas.set_width(width);
    canvas.set_height(height);

    let style = canvas.style();
    let device_scale = web_sys::window().unwrap().device_pixel_ratio();
    style
        .set_property("width", &((width as f64) / device_scale).to_string()[..])
        .unwrap();
    style
        .set_property("height", &((height as f64) / device_scale).to_string()[..])
        .unwrap();
}

fn init_input(param: &'static str, min: &str, max: &str, step: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let setter_id = format!("set-{}", param);
    let text = document.get_element_by_id(param).unwrap();
    let setter = document
        .get_element_by_id(&setter_id[..])
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    setter.set_min(min);
    if param == "webgl" {
        setter.set_max(
            &cmp::min(
                max.parse::<usize>().unwrap(),
                document
                    .get_element_by_id("canvas-webgl")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap()
                    .get_context("webgl")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<WebGlRenderingContext>()
                    .unwrap()
                    .get_parameter(WebGlRenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS)
                    .unwrap()
                    .as_f64()
                    .unwrap() as usize
                    / 4,
            )
            .to_string()[..],
        );
    } else {
        setter.set_max(max);
    }
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
        set_local_storage_item(param, value);
        restart_spectrum();
    }) as Box<dyn Fn()>);

    setter.set_onchange(Some(onchange.as_ref().unchecked_ref()));

    onchange.forget();
}

fn get_new_spectrum() {
    let width = get_local_storage_item("width").parse::<usize>().unwrap();
    let height = get_local_storage_item("height").parse::<usize>().unwrap();
    let num_sources = get_local_storage_item("num-sources")
        .parse::<usize>()
        .unwrap();
    let movement_speed = get_local_storage_item("movement-speed")
        .parse::<f32>()
        .unwrap();
    let color_speed = get_local_storage_item("color-speed")
        .parse::<f32>()
        .unwrap();

    match &get_local_storage_item("mode")[..] {
        "webgl" => unsafe {
            SPECTRUM = Some(Box::new(SpectrumGL::new(
                width,
                height,
                num_sources,
                movement_speed * MOVEMENT_SPEED_FACTOR,
                color_speed * COLOR_SPEED_FACTOR,
                get_canvas(),
            )));
        },
        _ => unsafe {
            SPECTRUM = Some(Box::new(SpectrumWasm::new(
                width,
                height,
                num_sources,
                movement_speed,
                color_speed,
                get_canvas(),
            )));
        },
    };
}

fn is_paused() -> bool {
    unsafe { FRAME != None }
}

fn pause() {
    if let Some(frame) = unsafe { FRAME } {
        let window = web_sys::window().unwrap();
        let play_pause_icon = window
            .document()
            .unwrap()
            .get_element_by_id("play-pause-icon")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        play_pause_icon.set_src("/static/play.svg");
        play_pause_icon.set_alt("Play");
        window.cancel_animation_frame(frame).unwrap();

        unsafe { FRAME = None };
    }
}

fn play() {
    if unsafe { FRAME } == None {
        let play_pause_icon = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("play-pause-icon")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
        play_pause_icon.set_src("/static/pause.svg");
        play_pause_icon.set_alt("Pause");
        web_sys::window()
            .unwrap()
            .request_animation_frame(
                Closure::wrap(Box::new(draw_frame) as Box<dyn Fn()>)
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();
    }
}

fn restart_spectrum() {
    let should_pause = is_paused();
    if should_pause {
        pause();
    }

    let document = web_sys::window().unwrap().document().unwrap();
    let mode = get_local_storage_item("mode");
    document
        .get_element_by_id(if mode == "webgl" {
            "canvas-wasm"
        } else {
            "canvas-webgl"
        })
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .class_list()
        .add_1("hide")
        .unwrap();

    document
        .get_element_by_id(if mode == "webgl" {
            "canvas-webgl"
        } else {
            "canvas-wasm"
        })
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .class_list()
        .remove_1("hide")
        .unwrap();

    get_new_spectrum();

    if should_pause {
        play();
    }
}

fn draw_frame() {
    unsafe {
        if let Some(spectrum) = &mut SPECTRUM {
            spectrum.draw();
            spectrum.tick();
            FRAME = Some(
                web_sys::window()
                    .unwrap()
                    .request_animation_frame(
                        Closure::wrap(Box::new(draw_frame) as Box<dyn Fn()>)
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .unwrap(),
            );
        }
    }
}

fn init_rendering_mode(setter_mode: &'static str) {
    let onclick = Closure::wrap(Box::new(move || {
        let current_mode = get_local_storage_item("mode");
        if current_mode != setter_mode {
            let document = web_sys::window().unwrap().document().unwrap();
            document
                .get_element_by_id(&format!("mode-{}", current_mode)[..])
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .remove_1("current-mode")
                .unwrap();

            document
                .get_element_by_id(&format!("mode-{}", setter_mode)[..])
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .remove_1("current-mode")
                .unwrap();
        }
    }) as Box<dyn Fn()>);

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(&format!("mode-{}", setter_mode)[..])
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(onclick.as_ref().unchecked_ref()));

    set_local_storage_item("mode", setter_mode);

    if !get_local_storage_item("lock").parse::<bool>().unwrap() {
        let State {
            width,
            height,
            num_sources,
            movement_speed,
            color_speed,
        } = get_default_params();
        set_local_storage_item("width", &width.to_string()[..]);
        set_local_storage_item("height", &height.to_string()[..]);
        set_local_storage_item("num-sources", &num_sources.to_string()[..]);
        set_local_storage_item("movement-speed", &movement_speed.to_string()[..]);
        set_local_storage_item("color-speed", &color_speed.to_string()[..]);
    }
    restart_spectrum();
}

fn init_listeners() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let max_width_str = max_width().to_string();

    let max_height_str = max_height().to_string();

    let mode = get_or_set_initial_value("mode", "webgl");
    // let lock = get_or_set_initial_value("lock", "false")
    //     .parse::<bool>()
    //     .unwrap();

    let State {
        width,
        height,
        num_sources,
        movement_speed,
        color_speed,
    } = get_default_params();

    let width = get_or_set_initial_value("width", width);
    let height = get_or_set_initial_value("height", height);
    get_or_set_initial_value("num-sources", num_sources);
    get_or_set_initial_value("movement-speed", movement_speed);
    get_or_set_initial_value("color-speed", color_speed);

    if mode == "webgl" {
        let context_webgl_options = Object::new();
        Reflect::set(
            &context_webgl_options,
            &"preserveDrawingBuffer".into(),
            &wasm_bindgen::JsValue::TRUE,
        )
        .unwrap();

        get_canvas()
            .get_context_with_context_options("webgl", &context_webgl_options)
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap()
            .viewport(
                0,
                0,
                width.parse::<i32>().unwrap(),
                height.parse::<i32>().unwrap(),
            );
    }

    resize_canvas();

    // let canvas_webgl = document
    //     .get_element_by_id("canvas-webgl")
    //     .unwrap()
    //     .dyn_into::<HtmlCanvasElement>()
    //     .unwrap();

    let canvas_2d = document
        .get_element_by_id("canvas-wasm")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
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

    let restart_onclick = Closure::wrap(Box::new(|| {
        get_new_spectrum();
    }) as Box<dyn Fn()>);
    document
        .get_element_by_id("restart-button")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(restart_onclick.as_ref().unchecked_ref()));
    restart_onclick.forget();

    let download_onclick = Closure::wrap(Box::new(|| {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("download-link")
            .unwrap()
            .dyn_into::<HtmlAnchorElement>()
            .unwrap()
            .set_href(
                &str::replace(
                    &get_canvas().to_data_url_with_type("image/png").unwrap()[..],
                    "image/png",
                    "image/octet-stream",
                )[..],
            );
    }) as Box<dyn Fn()>);

    document
        .get_element_by_id("download-link")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap()
        .set_onclick(Some(download_onclick.as_ref().unchecked_ref()));

    download_onclick.forget();

    init_rendering_mode("webgl");
    init_rendering_mode("wasm");
    init_rendering_mode("js");

    let unlock_onclick = Closure::wrap(Box::new(|| {
        if get_local_storage_item("lock").parse::<bool>().unwrap() {
            let document = web_sys::window().unwrap().document().unwrap();

            document
                .get_element_by_id("mode-lock")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .remove_1("current-mode")
                .unwrap();
            document
                .get_element_by_id("mode-unlock")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .add_1("current-mode")
                .unwrap();
        }
    }) as Box<dyn Fn()>);

    document
        .get_element_by_id("mode-unlock")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(unlock_onclick.as_ref().unchecked_ref()));

    unlock_onclick.forget();

    let lock_onclick = Closure::wrap(Box::new(|| {
        if !get_local_storage_item("lock").parse::<bool>().unwrap() {
            let document = web_sys::window().unwrap().document().unwrap();

            document
                .get_element_by_id("mode-unlock")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .remove_1("current-mode")
                .unwrap();
            document
                .get_element_by_id("mode-lock")
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .class_list()
                .add_1("current-mode")
                .unwrap();
        }
    }) as Box<dyn Fn()>);

    document
        .get_element_by_id("mode-lock")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(lock_onclick.as_ref().unchecked_ref()));

    lock_onclick.forget();

    init_input("width", MIN_DIMENSION, &max_width_str[..], "10");
    init_input("height", MIN_DIMENSION, &max_height_str[..], "10");
    init_input("num-sources", "2", "100", "1");
    init_input("movement-speed", "1", "100", "1");
    init_input("color-speed", "1", "100", "1");

    let collapse_onclick = Closure::wrap(Box::new(|| {
        let window = web_sys::window().unwrap();
        let document = web_sys::window().unwrap().document().unwrap();

        document
            .get_element_by_id("controls")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .class_list()
            .add_1("hide-controls")
            .unwrap();

        window
            .set_timeout_with_callback_and_timeout_and_arguments(
                Closure::wrap(Box::new(|| {
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("expand")
                        .unwrap()
                        .dyn_into::<HtmlElement>()
                        .unwrap()
                        .class_list()
                        .remove_1("hide-expand")
                        .unwrap();
                }) as Box<dyn Fn()>)
                .as_ref()
                .unchecked_ref(),
                500,
                &Array::new(),
            )
            .unwrap();
    }) as Box<dyn Fn()>);

    document
        .get_element_by_id("collapse")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(collapse_onclick.as_ref().unchecked_ref()));

    collapse_onclick.forget();

    let expand_onclick = Closure::wrap(Box::new(|| {
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("expand")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .class_list()
            .add_1("hide-expand")
            .unwrap();

        document
            .get_element_by_id("controls")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .class_list()
            .remove_1("hide-controls")
            .unwrap();
    }) as Box<dyn Fn()>);

    document
        .get_element_by_id("expand")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(expand_onclick.as_ref().unchecked_ref()));

    expand_onclick.forget();
}

#[wasm_bindgen(start)]
pub fn start() {
    init_listeners();

    let document = web_sys::window().unwrap().document().unwrap();

    document
        .get_element_by_id("canvas-webgl")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .class_list()
        .remove_1("hide")
        .unwrap();

    document
        .get_element_by_id("controls")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .class_list()
        .remove_1("hide")
        .unwrap();

    play();
}
