use wasm_bindgen::prelude::*;

// mod utils;

extern crate web_sys;
use js_sys::Math;
use std::f32::consts;
use std::iter;
// use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            Err(format!("provided hue({}) cannot be over 360", hue))
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
    x: usize,
    y: usize,
    hue_vectors: (f32, f32),
}

impl Source {
    pub fn new(width: usize, height: usize) -> Source {
        let x = (Math::random() * width as f64) as usize;
        let y = (Math::random() * height as f64) as usize;
        let hue = Hue::new((Math::random() * 360f64) as usize).unwrap();

        let hue_val = hue.get() as f32 * DEG_TO_RAD;
        // log_usize(x);
        // log_usize(y);
        // log_usize(hue.get());
        // log_usize(0);

        Source {
            x,
            y,
            hue_vectors: (hue_val.cos(), hue_val.sin()),
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
        self.hue_vectors
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
            for y in 0..self.height {
                let (mut hue_vector_cos, mut hue_vector_sin) = (0f32, 0f32);
                for source in &self.sources {
                    let (source_vector_cos, source_vector_sin) = source.hue_vectors();
                    let dist_factor =
                        ((x - source.x()).pow(2) + (y - source.y()).pow(2)) as f32 + 1f32;
                    hue_vector_cos += source_vector_cos / dist_factor;
                    hue_vector_sin += source_vector_sin / dist_factor;
                }

                let mut hue_val = (hue_vector_sin / hue_vector_cos).atan() * RAD_TO_DEG;

                if hue_vector_cos < 0f32 {
                    hue_val += 180f32;
                } else if hue_vector_sin < 0f32 {
                    hue_val += 360f32;
                    if hue_val >= 359.5f32 {
                        hue_val = 0f32;
                    }
                }

                // let hue = Hue::new(((x * width + y) % 360) as usize).expect(
                //     // &format!(
                //     //     "{}, {}, {}, {}",
                //     //     original, hue_val, hue_vector_cos, hue_vector_sin
                //     // )[..],
                //     "hello",
                // );
                // let rgba = hue.to_rgba();

                // if hue_val >= 265 && hue_val <= 300 {
                //     log("here");
                // }

                self.data[x + y * self.width] = Hue::new(hue_val as usize).unwrap().to_rgba();
            }
        }
    }

    pub fn data(&self) -> *const RGBA {
        self.data.as_slice().as_ptr()
    }
}
