use js_sys::Math;
use std::f32::consts;
use std::iter;

const TWO_PI: f32 = consts::PI * 2f32;
const TWO_THIRDS_PI: f32 = consts::FRAC_PI_3 * 2f32;
const FOUR_THIRDS_PI: f32 = consts::FRAC_PI_3 * 4f32;
const FIVE_THIRDS_PI: f32 = consts::FRAC_PI_3 * 5f32;

/// Wrapper of four byte values corresponding to RGBA for a single pixel.
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

/// Value in [0, 2π) corresponding to a hue value (in radians) in the HSL color space.
#[derive(Clone, Copy)]
pub struct Hue(f32);

impl Hue {
    /// Constructs a new Hue.
    ///
    /// # Arguments
    ///
    /// * `hue` - the new Hue value.
    pub fn new(hue: f32) -> Hue {
        Hue(hue)
    }

    /// Returns the internal f32 value corresponding to the Hue.
    pub fn get(self) -> f32 {
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
        self.0 += dh;
        if self.0 >= TWO_PI {
            self.0 -= TWO_PI;
        } else if self.0 <= 0f32 {
            self.0 += TWO_PI;
        }
    }

    /// Converts the Hue to its corresponding RGBA value.
    ///
    /// Sets saturation to 100% and lightness to 50% to get the Hue's truest color value.
    pub fn to_rgba(self) -> RGBA {
        let hue = self.0;
        if hue < consts::PI {
            if hue < consts::FRAC_PI_3 {
                RGBA(
                    u8::MAX,
                    (255f32 * hue / consts::FRAC_PI_3) as u8,
                    0,
                    u8::MAX,
                )
            } else if hue < TWO_THIRDS_PI {
                RGBA(
                    (255f32 * (2f32 - hue / consts::FRAC_PI_3)) as u8,
                    u8::MAX,
                    0,
                    u8::MAX,
                )
            } else {
                RGBA(
                    0,
                    u8::MAX,
                    (255f32 * (hue / consts::FRAC_PI_3 - 2f32)) as u8,
                    u8::MAX,
                )
            }
        } else if hue < FOUR_THIRDS_PI {
            RGBA(
                0,
                (255f32 * (4f32 - hue / consts::FRAC_PI_3)) as u8,
                u8::MAX,
                u8::MAX,
            )
        } else if hue < FIVE_THIRDS_PI {
            RGBA(
                (255f32 * (hue / consts::FRAC_PI_3 - 4f32)) as u8,
                0,
                u8::MAX,
                u8::MAX,
            )
        } else {
            RGBA(
                u8::MAX,
                0,
                (255f32 * (6f32 - hue / consts::FRAC_PI_3)) as u8,
                u8::MAX,
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
        movement_speed: f32,
        color_speed: f32,
    ) -> Source {
        let hue = Hue(Math::random() as f32 * TWO_PI);
        let hue_val = hue.get();
        let hue_cos = hue_val.cos();
        let hue_sin = hue_val.sin();
        Source {
            x: Math::random() as f32 * canvas_width,
            y: Math::random() as f32 * canvas_height,
            hue,
            canvas_width,
            canvas_height,
            dx: Math::random() as f32 * movement_speed - movement_speed / 2f32,
            dy: Math::random() as f32 * movement_speed - movement_speed / 2f32,
            dh: Math::random() as f32 * color_speed - color_speed / 2f32,
            hue_cos,
            hue_sin,
        }
    }

    /// Returns the x-coordinate of the Source.
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y-coordinate of the Source.
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Returns the cosine of the Source's hue.
    pub fn hue_cos(&self) -> f32 {
        self.hue_cos
    }

    /// Returns the sine of the Source's hue.
    pub fn hue_sin(&self) -> f32 {
        self.hue_sin
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

        if self.x <= 0f32 {
            self.x *= -1f32;
            self.dx *= -1f32;
        } else if self.x >= self.canvas_width {
            self.x = self.canvas_width - (self.x - self.canvas_width);
            self.dx *= -1f32;
        }

        if self.y <= 0f32 {
            self.y *= -1f32;
            self.dy *= -1f32;
        } else if self.y >= self.canvas_height {
            self.y = self.canvas_height - (self.y - self.canvas_height);
            self.dy *= -1f32;
        }
    }
}

/// The shared data belonging to both Spectrum implementations.
pub struct BaseSpectrum {
    /// The width of the Spectrum canvas.
    width: usize,

    /// The height of the Spectrum canvas.
    height: usize,

    /// A vector containing the Spectrum's sources.
    sources: Vec<Source>,
}

impl BaseSpectrum {
    /// Constructs a new BaseSpectrum.
    ///
    /// # Arguments
    ///
    /// * `width` - the width of the BaseSpectrum.
    /// * `height` - the height of the BaseSpectrum.
    /// * `num_sources` - the number of Sources to generate.
    /// * `movement_speed` - the range of each Source's movement speed (`dx`, `dy`)
    /// * `color_speed` - the range of each Source's color speed (`dh`)
    pub fn new(
        width: usize,
        height: usize,
        num_sources: usize,
        movement_speed: f32,
        color_speed: f32,
    ) -> BaseSpectrum {
        let width_float = width as f32;
        let height_float = height as f32;

        BaseSpectrum {
            width,
            height,
            sources: iter::repeat(())
                .map(|()| Source::new(width_float, height_float, movement_speed, color_speed))
                .take(num_sources)
                .collect(),
        }
    }

    /// Returns the width of the BaseSpectrum.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the BaseSpectrum.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the vector containing the BaseSpectrum's Sources.
    pub fn sources(&self) -> &Vec<Source> {
        &self.sources
    }

    /// Increments the BaseSpectrum's sources by one frame.
    pub fn tick(&mut self) {
        for source in &mut self.sources {
            source.tick();
        }
    }
}
