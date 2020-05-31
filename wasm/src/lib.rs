use wasm_bindgen::prelude::*;

// mod utils;

use js_sys::Math;
use std::f32::consts;
use std::iter;
// extern crate web_sys;

// use web_sys::console;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct RGBA(u8, u8, u8, u8);

impl RGBA {
    fn from_rgb(r: u8, g: u8, b: u8) -> RGBA {
        RGBA(r, g, b, 255)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
struct Hue(f32);

impl Hue {
    fn get(self) -> f32 {
        self.0
    }

    fn tick(&mut self, dh: f32) {
        self.0 += dh;
        if self.0 >= 360f32 {
            self.0 -= 360f32;
        } else if self.0 <= 0f32 {
            self.0 += 360f32;
        }
    }

    fn to_rgba(self) -> RGBA {
        let hue = self.0;
        let primary = 255;
        let secondary = ((1f32 - ((hue / 60f32) % 2f32 - 1f32).abs()) * 255f32) as u8;
        if hue < 180f32 {
            if hue < 60f32 {
                RGBA::from_rgb(primary, secondary, 0)
            } else if hue < 120f32 {
                RGBA::from_rgb(secondary, primary, 0)
            } else {
                RGBA::from_rgb(0, primary, secondary)
            }
        } else if hue < 240f32 {
            RGBA::from_rgb(0, secondary, primary)
        } else if hue < 300f32 {
            RGBA::from_rgb(secondary, 0, primary)
        } else {
            RGBA::from_rgb(primary, 0, secondary)
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
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    dh: f32,

    hue: Hue,
    hue_vectors: (f32, f32),
}

impl Source {
    pub fn new(width: f32, height: f32) -> Source {
        let x = Math::random() as f32 * width;
        let y = Math::random() as f32 * height;
        let dx = Math::random() as f32 * 2f32 - 1f32;
        let dy = Math::random() as f32 * 2f32 - 1f32;
        let dh = Math::random() as f32 * 6f32 - 3f32;
        let hue = Hue(Math::random() as f32 * 360f32);

        let hue_val = hue.get().to_radians();
        Source {
            x,
            y,
            dx,
            dy,
            dh,
            hue,
            hue_vectors: (hue_val.cos(), hue_val.sin()),
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn hue_vectors(&self) -> (f32, f32) {
        self.hue_vectors
    }

    pub fn tick(&mut self, width: f32, height: f32) {
        self.hue.tick(self.dh);
        let hue_rad = self.hue.get().to_radians();
        self.hue_vectors = (hue_rad.cos(), hue_rad.sin());

        self.x += self.dx;
        self.y += self.dy;

        if self.x <= 0f32 {
            self.x *= -1f32;
            self.dx *= -1f32;
        } else if self.x >= width {
            self.x = width - (self.x - width);
            self.dx *= -1f32;
        }

        if self.y <= 0f32 {
            self.y *= -1f32;
            self.dy *= -1f32;
        } else if self.y >= height {
            self.y = height - (self.y - height);
            self.dy *= -1f32;
        }
    }
}

#[wasm_bindgen]
pub struct Spectrum {
    width: usize,
    height: usize,
    sources: Vec<Source>,
    data: Vec<RGBA>,
}

fn atan(quotient: f32) -> f32 {
    // const COEFF_A: f32 = 0.972_394_1;
    // const COEFF_B: f32 = -0.191_947_95;
    // (COEFF_A + COEFF_B * normalized * normalized) * normalized

    (consts::FRAC_PI_4 + 0.273f32 * (1f32 - quotient.abs())) * quotient
}

fn atan2(x: f32, y: f32) -> f32 {
    if x.abs() > y.abs() {
        let quotient = y / x;
        if x < 0f32 {
            atan(quotient) + consts::PI
        } else if y < 0f32 {
            atan(quotient) + 2f32 * consts::PI
        } else {
            atan(quotient)
        }
    } else {
        let quotient = x / y;
        if y < 0f32 {
            -atan(quotient) + 3f32 * consts::FRAC_PI_2
        } else {
            -atan(quotient) + consts::FRAC_PI_2
        }
    }
}

#[wasm_bindgen]
impl Spectrum {
    pub fn new(width: usize, height: usize, num_sources: usize) -> Spectrum {
        let width_float = width as f32;
        let height_float = height as f32;
        let sources: Vec<Source> = iter::repeat(())
            .map(|()| Source::new(width_float, height_float))
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

        for x in 0..self.width {
            let x_float = x as f32;
            for y in 0..self.height {
                let y_float = y as f32;
                let (mut hue_vector_cos, mut hue_vector_sin) = (0f32, 0f32);
                for source in &self.sources {
                    let (source_vector_cos, source_vector_sin) = source.hue_vectors();
                    let dist_factor =
                        (x_float - source.x()).powi(2) + (y_float - source.y()).powi(2) + 1f32;
                    hue_vector_cos += source_vector_cos / dist_factor;
                    hue_vector_sin += source_vector_sin / dist_factor;
                }

                self.data[x + y * self.width] =
                    Hue(atan2(hue_vector_cos, hue_vector_sin).to_degrees()).to_rgba();
            }
        }
    }

    pub fn tick(&mut self) {
        // utils::set_panic_hook();

        let width_float = self.width as f32;
        let height_float = self.height as f32;

        for source in &mut self.sources {
            source.tick(width_float, height_float);
        }
    }

    pub fn data(&self) -> *const RGBA {
        self.data.as_slice().as_ptr()
    }
}
