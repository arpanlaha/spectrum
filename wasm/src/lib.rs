use wasm_bindgen::prelude::*;

mod utils;

// extern crate web_sys;
use js_sys::Math;
use std::f64::consts;
use std::iter;
// use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const RAD_TO_DEG: f64 = 180f64 / consts::PI;
const DEG_TO_RAD: f64 = consts::PI / 180f64;

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
struct Hue(f64);

impl Hue {
    fn new(hue: f64) -> Result<Hue, String> {
        if hue < 360f64 {
            Ok(Hue(hue))
        } else {
            Err(format!("provided hue({}) cannot be over 360", hue))
        }
    }

    fn get(self) -> f64 {
        self.0
    }

    fn to_rgba(self) -> RGBA {
        let hue = self.0;
        let primary = 255;
        let secondary = ((1f64 - ((hue / 60f64) % 2f64 - 1f64).abs()) * 255f64) as u8;
        if hue < 180f64 {
            if hue < 60f64 {
                RGBA::from_rgb(primary, secondary, 0)
            } else if hue < 120f64 {
                RGBA::from_rgb(secondary, primary, 0)
            } else {
                RGBA::from_rgb(0, primary, secondary)
            }
        } else if hue < 240f64 {
            RGBA::from_rgb(0, secondary, primary)
        } else if hue < 300f64 {
            RGBA::from_rgb(secondary, 0, primary)
        } else {
            RGBA::from_rgb(primary, 0, secondary)
        }

        // match hue {
        //     0f64 => RGBA::from_rgb(primary, secondary, 0),
        //     1f64 => RGBA::from_rgb(secondary, primary, 0),
        //     2f64 => RGBA::from_rgb(0, primary, secondary),
        //     3f64 => RGBA::from_rgb(0, secondary, primary),
        //     4f64 => RGBA::from_rgb(secondary, 0, primary),
        //     _ => RGBA::from_rgb(primary, 0, secondary),
        // }
    }
}

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_usize(a: usize);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_f32(a: f32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_f32_pair(a: f32, b: f32);
// }

#[wasm_bindgen]
struct Source {
    x: f64,
    y: f64,
    hue: Hue,
    hue_vectors: (f64, f64),
}

impl Source {
    pub fn new(width: usize, height: usize) -> Source {
        let x = Math::random() * width as f64;
        let y = Math::random() * height as f64;
        let hue = Hue::new(Math::random() * 360f64).unwrap();

        let hue_val = hue.get() * DEG_TO_RAD;
        // log_usize(x);
        // log_usize(y);
        // log_usize(hue.get());
        // log_usize(0);

        Source {
            x,
            y,
            hue,
            hue_vectors: (hue_val.cos(), hue_val.sin()),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    // pub fn hue(&self) -> Hue {
    //     self.hue
    // }

    pub fn hue_vectors(&self) -> (f64, f64) {
        self.hue_vectors
    }

    pub fn tick(&mut self) {
        let hue_val = (self.hue.get() + 5f64) % 360f64;
        self.hue = Hue::new(hue_val).unwrap();
        let hue_rad = hue_val * DEG_TO_RAD;
        self.hue_vectors = (hue_rad.cos(), hue_rad.sin());
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
            .map(|()| Source::new(width, height))
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
        // utils::set_panic_hook();

        // let width = self.width;
        // let height = self.height;

        // log_usize(width);
        // log_usize(height);

        for x in 0..self.width {
            let x_float = x as f64;
            for y in 0..self.height {
                let y_float = y as f64;
                let (mut hue_vector_cos, mut hue_vector_sin) = (0f64, 0f64);
                for source in &self.sources {
                    let (source_vector_cos, source_vector_sin) = source.hue_vectors();
                    let dist_factor = (x_float - source.x()).powf(2f64)
                        + (y_float - source.y()).powf(2f64)
                        + 1f64;
                    hue_vector_cos += source_vector_cos / dist_factor;
                    hue_vector_sin += source_vector_sin / dist_factor;
                }

                let mut hue_val = (hue_vector_sin / hue_vector_cos).atan() * RAD_TO_DEG;

                if hue_vector_cos < 0f64 {
                    hue_val += 180f64;
                } else if hue_vector_sin < 0f64 {
                    hue_val += 360f64;
                    if hue_val >= 359.5f64 {
                        hue_val = 0f64;
                    }
                }

                self.data[x + y * self.width] = Hue::new(hue_val).unwrap().to_rgba();
            }
        }
    }

    pub fn tick(&mut self) {
        utils::set_panic_hook();

        for source in &mut self.sources {
            source.tick();
        }
    }

    pub fn data(&self) -> *const RGBA {
        self.data.as_slice().as_ptr()
    }
}
