use std::u8;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::utils::base::SOURCE_DROPOFF_FACTOR;
use crate::utils::base::{BaseSpectrum, Hue, RGB};
use crate::utils::math;

/// A WebAssembly-only implementation of Spectrum.
#[wasm_bindgen]
pub struct SpectrumWasm {
    /// The Spectrum's BaseSpectrum.
    base: BaseSpectrum,

    /// The Spectrum's pixel data.
    data: Vec<u8>,

    /// The `2d` context belonging to the Spectrum's canvas.
    context: CanvasRenderingContext2d,

    source_dropoff: f32,
}

#[wasm_bindgen]
impl SpectrumWasm {
    /// Creates a new Spectrum.
    ///
    /// # Arguments
    ///
    /// * `width` - the Spectrum's width.
    /// * `height` - the Spectrum's height.
    /// * `num_sources` - the number of Sources in the Spectrum.
    /// * `context` - the `2d` context belonging to the Spectrum's canvas.
    /// * `movement_speed` - the range of each Source's movement speed (`dx`, `dy`)
    /// * `color_speed` - the range of each Source's color speed (`dh`)
    #[must_use]
    pub fn new(
        width: u32,
        height: u32,
        num_sources: u32,
        movement_speed: u32,
        color_speed: u32,
        source_dropoff: f32,
        canvas: &HtmlCanvasElement,
    ) -> Self {
        let mut spectrum = Self {
            base: BaseSpectrum::new(width, height, num_sources, movement_speed, color_speed),
            data: vec![0_u8; (width * height * 4) as usize],
            context: canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap(),
            source_dropoff: ((source_dropoff as f32) * SOURCE_DROPOFF_FACTOR).powf(2.),
        };
        spectrum.draw();

        spectrum
    }

    /// Draws to the Spectrum canvas, using the Spectrum's context to put the resulting ImageData.
    ///
    /// Assigns Hues to each pixel based off of an average inverse square distance weighting across all Sources.
    ///
    /// As hue in HSL is a circular/periodic metric, a numerical average is inaccurate - instead, hue is broken into sine and cosine components which are summed and reconstructed into the resulting Hue.
    pub fn draw(&mut self) {
        let width = self.base.width();
        for y in 0..self.base.height() {
            let y_float = y as f32;
            let y_by_width = y * width;

            for x in 0..width {
                let mut dist_factor_inverse_sum: f32 = 0.;

                let (hue_vector_cos, hue_vector_sin) =
                    self.base
                        .sources()
                        .iter()
                        .fold((0., 0.), |(sum_cos, sum_sin), source| {
                            let x_diff = x as f32 - source.x();
                            let y_diff = y_float - source.y();

                            let dist_factor = (x_diff).mul_add(x_diff, y_diff * y_diff) + 1.;
                            dist_factor_inverse_sum += 1. / dist_factor;

                            (
                                sum_cos + source.hue_cos() / dist_factor,
                                sum_sin + source.hue_sin() / dist_factor,
                            )
                        });

                dist_factor_inverse_sum = dist_factor_inverse_sum.min(1.);
                let adjusted_dist_factor_inverse_sum =
                    dist_factor_inverse_sum.powf(self.source_dropoff);
                let alpha = (f32::from(u8::MAX) * adjusted_dist_factor_inverse_sum) as u8;

                let RGB(r, g, b) =
                    Hue::new(math::atan2_approx(hue_vector_cos, hue_vector_sin)).to_rgb();

                let start = ((x + y_by_width) * 4) as usize;

                unsafe {
                    *self.data.get_unchecked_mut(start) = r;
                    *self.data.get_unchecked_mut(start + 1) = g;
                    *self.data.get_unchecked_mut(start + 2) = b;
                    *self.data.get_unchecked_mut(start + 3) = alpha;
                }
            }
        }

        self.context
            .put_image_data(
                &ImageData::new_with_u8_clamped_array(
                    wasm_bindgen::Clamped(self.data.as_mut_slice()),
                    width,
                )
                .unwrap(),
                0_f64,
                0_f64,
            )
            .unwrap();
    }

    /// Increments all of the Spectrum's sources by one frame.
    pub fn tick(&mut self) {
        self.base.tick();
    }
}
