use crate::utils::panic;
use rand::{rngs::OsRng, Rng};
use std::f32::consts;
use std::iter;

const TWO_PI: f32 = consts::PI * 2_f32;
const TWO_THIRDS_PI: f32 = consts::FRAC_PI_3 * 2_f32;
const FOUR_THIRDS_PI: f32 = consts::FRAC_PI_3 * 4_f32;
const FIVE_THIRDS_PI: f32 = consts::FRAC_PI_3 * 5_f32;
pub const SOURCE_DROPOFF_FACTOR: f32 = 0.01;
const MOVEMENT_SPEED_FACTOR: f32 = 0.2;
const COLOR_SPEED_FACTOR: f32 = 0.002;

/// Wrapper of three byte values corresponding to RGB for a single pixel.
pub struct RGB(pub u8, pub u8, pub u8);

/// Value in [0, 2π) corresponding to a hue value (in radians) in the HSL color space.
#[derive(Clone, Copy)]
pub struct Hue(f32);

impl Hue {
    /// Constructs a new Hue.
    ///
    /// # Arguments
    ///
    /// * `hue` - the new Hue value.
    pub const fn new(hue: f32) -> Self {
        Self(hue)
    }

    /// Returns the internal f32 value corresponding to the Hue.
    pub const fn get(self) -> f32 {
        self.0
    }

    /// Increments the internal value by the specificied delta.
    ///
    /// If the new value lies outside the valid Hue range, it is adjusted accordingly by one period.
    ///
    /// # Arguments
    ///
    /// * `dh` - the desired change to the internal value.
    fn tick(&mut self, dh: f32) {
        self.0 = (self.0 + dh) % TWO_PI;
    }

    /// Converts the Hue to its corresponding RGB value.
    ///
    /// Sets saturation to 100% and lightness to 50% to get the Hue's truest color value.
    ///
    /// Derived from [`RapidTables` HSL to RGB color conversion](https://www.rapidtables.com/convert/color/hsl-to-rgb.html).
    pub fn to_rgb(self) -> RGB {
        let hue = self.0;
        if hue < consts::PI {
            if hue < consts::FRAC_PI_3 {
                RGB(u8::MAX, (255_f32 * hue / consts::FRAC_PI_3) as u8, 0)
            } else if hue < TWO_THIRDS_PI {
                RGB(
                    (255_f32 * (2_f32 - hue / consts::FRAC_PI_3)) as u8,
                    u8::MAX,
                    0,
                )
            } else {
                RGB(
                    0,
                    u8::MAX,
                    (255_f32 * (hue / consts::FRAC_PI_3 - 2_f32)) as u8,
                )
            }
        } else if hue < FOUR_THIRDS_PI {
            RGB(
                0,
                (255_f32 * (4_f32 - hue / consts::FRAC_PI_3)) as u8,
                u8::MAX,
            )
        } else if hue < FIVE_THIRDS_PI {
            RGB(
                (255_f32 * (hue / consts::FRAC_PI_3 - 4_f32)) as u8,
                0,
                u8::MAX,
            )
        } else {
            RGB(
                u8::MAX,
                0,
                (255_f32 * (6_f32 - hue / consts::FRAC_PI_3)) as u8,
            )
        }
    }
}

/// A Source in the Spectrum canvas which influences the color of neighboring pixels.
pub struct Source {
    /// The x-coordinate of the Source in the Spectrum canvas.
    x: f32,

    /// The y-coordinate of the Source in the Spectrum canvas.
    y: f32,

    /// The internal Hue value of the Source.
    hue: Hue,

    /// The width of the Spectrum canvas.
    canvas_width: f32,

    /// The height of the spectrum canvas:
    canvas_height: f32,

    /// The rate of movement in the x direction.
    dx: f32,

    /// The rate of movement in the y direction.
    dy: f32,

    /// The rate of change in the Source's Hue.
    dh: f32,

    /// The cosine of the internal Hue value.
    hue_cos: f32,

    /// The sine of the internal Hue value.
    hue_sin: f32,

    dx_random: f32,
    dy_random: f32,
    dh_random: f32,
}

fn get_speed(input: f32, random: f32) -> f32 {
    input.mul_add(random, -input / 2.)
}

impl Source {
    /// Constructs a new Source.
    ///
    /// Non-specified paramaters are generated at random.
    ///
    /// # Arguments
    ///
    /// * `canvas_width` - the width of the Spectrum canvas.
    /// * `canvas_height` - the height of the Spectrum canvas.
    /// * `movement_speed` - the range of the Source's movement speed (`dx`, `dy`)
    /// * `color_speed` - the range of the Source's color speed (`dh`)
    pub fn new(
        canvas_width: f32,
        canvas_height: f32,
        movement_speed: u32,
        color_speed: u32,
    ) -> Self {
        let hue = Hue(OsRng.gen_range(0.0_f32..TWO_PI));
        let hue_val = hue.get();
        let hue_cos = hue_val.cos();
        let hue_sin = hue_val.sin();

        let movement_speed_float = (movement_speed as f32) * MOVEMENT_SPEED_FACTOR;
        let color_speed_float = (color_speed as f32) * COLOR_SPEED_FACTOR;

        let dx_random: f32 = OsRng.gen();
        let dy_random: f32 = OsRng.gen();
        let dh_random: f32 = OsRng.gen();

        Self {
            x: OsRng.gen_range(0.0_f32..canvas_width),
            y: OsRng.gen_range(0.0_f32..canvas_height),

            hue,
            canvas_width,
            canvas_height,
            dx_random,
            dy_random,
            dh_random,
            dx: get_speed(movement_speed_float, dx_random),
            dy: get_speed(movement_speed_float, dy_random),
            dh: get_speed(color_speed_float, dh_random),
            hue_cos,
            hue_sin,
        }
    }

    /// Returns the x-coordinate of the Source.
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y-coordinate of the Source.
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Returns the cosine of the Source's hue.
    pub const fn hue_cos(&self) -> f32 {
        self.hue_cos
    }

    /// Returns the sine of the Source's hue.
    pub const fn hue_sin(&self) -> f32 {
        self.hue_sin
    }

    pub fn update_movement_speed(&mut self, movement_speed: u32) {
        let movement_speed_float = (movement_speed as f32) * MOVEMENT_SPEED_FACTOR;

        self.dx = self.dx.signum() * get_speed(movement_speed_float, self.dx_random).abs();
        self.dy = self.dy.signum() * get_speed(movement_speed_float, self.dy_random).abs();
    }

    pub fn update_color_speed(&mut self, color_speed: u32) {
        let color_speed_float = (color_speed as f32) * COLOR_SPEED_FACTOR;

        self.dh = self.dh.signum() * get_speed(color_speed_float, self.dh_random).abs();
    }

    /// Increments the Source by one frame.
    ///
    /// The internal hue is incremented by the Source's `dh` value.
    ///
    /// The Source's position is incremented by `dx` and `dy`, with border collisions behaving as a bounce.
    fn tick(&mut self) {
        self.hue.tick(self.dh);
        let hue_val = self.hue.get();
        self.hue_cos = hue_val.cos();
        self.hue_sin = hue_val.sin();

        self.x += self.dx;
        self.y += self.dy;

        if self.x <= 0_f32 {
            self.x *= -1_f32;
            self.dx *= -1_f32;
        } else if self.x >= self.canvas_width {
            self.x = self.canvas_width - (self.x - self.canvas_width);
            self.dx *= -1_f32;
        }

        if self.y <= 0_f32 {
            self.y *= -1_f32;
            self.dy *= -1_f32;
        } else if self.y >= self.canvas_height {
            self.y = self.canvas_height - (self.y - self.canvas_height);
            self.dy *= -1_f32;
        }
    }
}

/// The shared data belonging to both Spectrum implementations.
pub struct BaseSpectrum {
    /// The width of the Spectrum canvas.
    width: u32,

    /// The height of the Spectrum canvas.
    height: u32,

    /// A vector containing the Spectrum's sources.
    sources: Vec<Source>,
}

impl BaseSpectrum {
    /// Constructs a new `BaseSpectrum`.
    ///
    /// # Arguments
    ///
    /// * `width` - the width of the `BaseSpectrum`.
    /// * `height` - the height of the `BaseSpectrum`.
    /// * `num_sources` - the number of Sources to generate.
    /// * `movement_speed` - the range of each Source's movement speed (`dx`, `dy`)
    /// * `color_speed` - the range of each Source's color speed (`dh`)
    pub fn new(
        width: u32,
        height: u32,
        num_sources: u32,
        movement_speed: u32,
        color_speed: u32,
    ) -> Self {
        panic::set_panic_hook();

        let width_float = width as f32;
        let height_float = height as f32;

        Self {
            width,
            height,
            sources: iter::repeat(())
                .map(|()| Source::new(width_float, height_float, movement_speed, color_speed))
                .take(num_sources as usize)
                .collect(),
        }
    }

    /// Returns the width of the `BaseSpectrum`.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the `BaseSpectrum`.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns a reference to the vector containing the `BaseSpectrum`'s Sources.
    pub const fn sources(&self) -> &Vec<Source> {
        &self.sources
    }

    pub fn update_movement_speed(&mut self, movement_speed: u32) {
        for source in &mut self.sources {
            source.update_movement_speed(movement_speed);
        }
    }

    pub fn update_color_speed(&mut self, color_speed: u32) {
        for source in &mut self.sources {
            source.update_color_speed(color_speed);
        }
    }

    /// Increments the `BaseSpectrum`'s sources by one frame.
    pub fn tick(&mut self) {
        for source in &mut self.sources {
            source.tick();
        }
    }
}
