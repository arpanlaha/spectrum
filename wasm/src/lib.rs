use wasm_bindgen::prelude::*;

mod utils;

extern crate web_sys;
use js_sys::Math;
use std::f32::consts;
use std::iter;
// use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const RAD_TO_DEG: f32 = 180f32 / consts::PI;
const DEG_TO_RAD: f32 = consts::PI / 180f32;

// #[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct RGBA(u8, u8, u8, u8);

impl RGBA {
    fn from_rgb(r: u8, g: u8, b: u8) -> RGBA {
        RGBA(r, g, b, 255)
    }
}

// #[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
struct Hue(usize);

impl Hue {
    fn new(hue: usize) -> Result<Hue, String> {
        if hue < 360 {
            Ok(Hue(hue))
        } else {
            Err(String::from("Hue cannot be 360 or above"))
        }
    }

    fn get(self) -> usize {
        self.0
    }

    fn to_rgba(self) -> RGBA {
        let hue = self.0;
        let primary = 255;
        let secondary = ((1f32 - ((hue as f32 / 60f32) % 2f32 - 1f32).abs()) * 255f32) as u8;
        match hue / 60 {
            0 => RGBA::from_rgb(primary, secondary, 0),
            1 => RGBA::from_rgb(secondary, primary, 0),
            2 => RGBA::from_rgb(0, primary, secondary),
            3 => RGBA::from_rgb(0, secondary, primary),
            4 => RGBA::from_rgb(secondary, 0, primary),
            _ => RGBA::from_rgb(primary, 0, secondary),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(a: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(a: f32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32_pair(a: f32, b: f32);
}

#[wasm_bindgen]
struct Source {
    x: usize,
    y: usize,
    hue: Hue,
}

impl Source {
    pub fn new(width: usize, height: usize, hue: Hue) -> Source {
        Source {
            x: (Math::random() * width as f64) as usize,
            y: (Math::random() * height as f64) as usize,
            hue,
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    // pub fn hue(&self) -> Hue {
    //     self.hue
    // }

    pub fn hue_vectors(&self) -> (f32, f32) {
        let hue_val = self.hue.get() as f32 * DEG_TO_RAD;
        (hue_val.cos(), hue_val.sin())
    }
}

#[wasm_bindgen]
pub struct Spectrum {
    width: usize,
    height: usize,
    sources: Vec<Source>,
    // canvas: web_sys::HtmlCanvasElement,
    data: Vec<RGBA>,
}

#[wasm_bindgen]
impl Spectrum {
    pub fn new(width: usize, height: usize, num_sources: usize) -> Spectrum {
        let sources: Vec<Source> = iter::repeat(())
            .map(|()| {
                Source::new(
                    width,
                    height,
                    Hue::new((Math::random() * 360f64) as usize).unwrap(),
                )
            })
            .take(num_sources)
            .collect();
        let mut spectrum = Spectrum {
            width,
            height,
            sources,
            data: vec![RGBA::from_rgb(0, 0, 0); width * height],
        };
        spectrum.draw();
        spectrum
    }

    pub fn draw(&mut self) {
        utils::set_panic_hook();

        let width = self.width;
        let height = self.height;
        for x in 0..width {
            for y in 0..height {
                let (mut hue_vector_x, mut hue_vector_y) = (0f32, 0f32);
                for source in &self.sources {
                    let (source_vector_x, source_vector_y) = source.hue_vectors();
                    hue_vector_x += source_vector_x / (((x - source.x()) as f32).abs() + 1f32);
                    hue_vector_y += source_vector_y / (((y - source.y()) as f32).abs() + 1f32);
                }
                let hue = Hue::new(
                    (hue_vector_y.atan2(hue_vector_x) % (2f32 * consts::PI) * RAD_TO_DEG) as usize,
                )
                .unwrap();
                let rgba = hue.to_rgba();

                self.data[x * width + y] = rgba;
            }
        }
    }

    pub fn data(&self) -> *const RGBA {
        self.data.as_slice().as_ptr()
    }
}
