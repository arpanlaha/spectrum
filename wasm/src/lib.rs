use js_sys::Math;
use std::f32::consts;
use std::iter;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, ImageData, WebGlProgram, WebGlRenderingContext, WebGlShader,
};

const TWO_PI: f32 = consts::PI * 2f32;
const TWO_THIRDS_PI: f32 = consts::FRAC_PI_3 * 2f32;
const FOUR_THIRDS_PI: f32 = consts::FRAC_PI_3 * 4f32;
const FIVE_THIRDS_PI: f32 = consts::FRAC_PI_3 * 5f32;

/// Wrapper of four byte values corresponding to RGBA for a single pixel.
pub struct RGBA(u8, u8, u8, u8);

/// Value in [0, 2π) corresponding to a hue value (in radians) in the HSL color space.
#[derive(Clone, Copy)]
struct Hue(f32);

impl Hue {
    /// Returns the internal f32 value corresponding to the Hue.
    fn get(self) -> f32 {
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
    fn to_rgba(self) -> RGBA {
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
struct Source {
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
    fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y-coordinate of the Source.
    fn y(&self) -> f32 {
        self.y
    }

    /// Returns the cosine of the Source's hue.
    fn hue_cos(&self) -> f32 {
        self.hue_cos
    }

    /// Returns the sine of the Source's hue.
    fn hue_sin(&self) -> f32 {
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
struct BaseSpectrum {
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
        BaseSpectrum {
            width,
            height,
            sources: iter::repeat(())
                .map(|()| Source::new(width as f32, height as f32, movement_speed, color_speed))
                .take(num_sources)
                .collect(),
        }
    }

    /// Increments the BaseSpectrum's sources by one frame.
    fn tick(&mut self) {
        for source in &mut self.sources {
            source.tick();
        }
    }
}

/// A WebAssembly-only implementation of Spectrum.
#[wasm_bindgen]
pub struct Spectrum {
    /// The Spectrum's BaseSpectrum.
    base: BaseSpectrum,

    /// The Spectrum's pixel data.
    data: Vec<u8>,

    /// The `2d` context belonging to the Spectrum's canvas.
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Spectrum {
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
    pub fn new(
        width: usize,
        height: usize,
        num_sources: usize,
        movement_speed: f32,
        color_speed: f32,
        context: CanvasRenderingContext2d,
    ) -> Spectrum {
        let mut spectrum = Spectrum {
            base: BaseSpectrum::new(width, height, num_sources, movement_speed, color_speed),
            data: vec![0u8; width * height * 4],
            context,
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
        for x in 0..self.base.width {
            let x_float = x as f32;
            for y in 0..self.base.height {
                let y_float = y as f32;
                let (mut hue_vector_cos, mut hue_vector_sin) = (0f32, 0f32);
                for source in &self.base.sources {
                    let dist_factor =
                        (x_float - source.x()).powi(2) + (y_float - source.y()).powi(2) + 1f32;
                    hue_vector_cos += source.hue_cos() / dist_factor;
                    hue_vector_sin += source.hue_sin() / dist_factor;
                }

                let RGBA(r, g, b, a) = Hue(atan2(hue_vector_cos, hue_vector_sin)).to_rgba();

                let start = (x + y * self.base.width) * 4;

                self.data[start] = r;
                self.data[start + 1] = g;
                self.data[start + 2] = b;
                self.data[start + 3] = a;
            }
        }

        self.context
            .put_image_data(
                &ImageData::new_with_u8_clamped_array(
                    wasm_bindgen::Clamped(self.data.as_mut_slice()),
                    self.base.width as u32,
                )
                .unwrap(),
                0f64,
                0f64,
            )
            .unwrap();
    }

    /// Increments all of the Spectrum's sources by one frame.
    pub fn tick(&mut self) {
        self.base.tick();
    }
}

#[wasm_bindgen]
pub struct SpectrumGL {
    base: BaseSpectrum,
    context: WebGlRenderingContext,
    program: WebGlProgram,
}

#[wasm_bindgen]
impl SpectrumGL {
    /// Creates a new SpectrumGl.
    ///
    /// # Arguments
    ///
    /// * `width` - the SpectrumGL's width.
    /// * `height` - the SpectrumGL's height.
    /// * `num_sources` - the number of Sources in the SpectrumGL.
    /// * `context` - the `webgl` context belonging to the SpectrumGL's canvas.
    /// * `movement_speed` - the range of each Source's movement speed (`dx`, `dy`)
    /// * `color_speed` - the range of each Source's color speed (`dh`)
    pub fn new(
        width: usize,
        height: usize,
        num_sources: usize,
        movement_speed: f32,
        color_speed: f32,
        context: WebGlRenderingContext,
    ) -> SpectrumGL {
        let vertex_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
                attribute vec4 a_position;

                void main(void) {
                    gl_Position = a_position;
                }
            "#,
        );

        let fragment_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            &format!(
                r#"
                    #define PI 3.141592653589793
                    #define PI_3 1.0471975511965976
                    #define PI_2_3 2.0943951023931953
                    #define PI_4_3 4.1887902047863905
                    #define PI_5_3 5.235987755982989

                    precision highp float;

                    uniform float sources[{}];

                    void main() {{
                        float x = gl_FragCoord[0];
                        float y = gl_FragCoord[1];
                        float cos_sum = 0.0;
                        float sin_sum = 0.0;

                        for (int i = 0; i < {}; i++) {{
                            float dist_factor = pow(sources[4 * i] - x, 2.0) + pow(sources[4 * i + 1] - y, 2.0) + 1.0;
                            cos_sum += sources[4 * i + 2] / dist_factor;
                            sin_sum += sources[4 * i + 3] / dist_factor;
                        }}

                        float hue = atan(sin_sum, cos_sum);
                        if (hue < 0.0) {{
                            hue += 2.0 * PI;
                        }}
                        float secondary = 1.0 - abs(mod((hue / PI_3), 2.0) - 1.0);

                        if (hue < PI) {{
                            if (hue < PI_3) {{
                                gl_FragColor = vec4(1.0, secondary, 0.0, 1.0);
                            }} else if (hue < PI_2_3) {{
                                gl_FragColor = vec4(secondary, 1.0, 0.0, 1.0);
                            }} else {{
                                gl_FragColor = vec4(0.0, 1.0, secondary, 1.0);
                            }}
                        }} else if (hue < PI_4_3) {{
                            gl_FragColor = vec4(0.0, secondary, 1.0, 1.0);
                        }} else if (hue < PI_5_3) {{
                            gl_FragColor = vec4(secondary, 0.0, 1.0, 1.0);
                        }} else {{
                            gl_FragColor = vec4(1.0, 0.0, secondary, 1.0);
                        }}
                    }}
                "#,
                num_sources * 4,
                num_sources
            )[..],
        );

        let program = context.create_program().unwrap();

        context.attach_shader(&program, &vertex_shader);
        context.attach_shader(&program, &fragment_shader);
        context.link_program(&program);
        context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap();

        context.use_program(Some(&program));

        let position_attribute_loc = context.get_attrib_location(&program, "a_position") as u32;

        let vertex_coords = [-1f32, -1f32, 1f32, -1f32, 1f32, 1f32, -1f32, 1f32];

        let buffer = context.create_buffer().unwrap();
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vertex_array = js_sys::Float32Array::view(&vertex_coords);

            context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        context.enable_vertex_attrib_array(position_attribute_loc);

        context.vertex_attrib_pointer_with_i32(
            position_attribute_loc,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let spectrum = SpectrumGL {
            base: BaseSpectrum::new(width, height, num_sources, movement_speed, color_speed),
            context,
            program,
        };
        spectrum.draw();

        spectrum
    }

    /// Draws to the Spectrum canvas, adjusting the context's shaders to match the current state.
    ///
    /// Assigns Hues to each pixel based off of an average inverse square distance weighting across all Sources.
    ///
    /// As hue in HSL is a circular/periodic metric, a numerical average is inaccurate - instead, hue is broken into sine and cosine components which are summed and reconstructed into the resulting Hue.
    pub fn draw(&self) {
        let source_info: Vec<f32> = self
            .base
            .sources
            .iter()
            .map(|source| vec![source.x(), source.y(), source.hue_cos(), source.hue_sin()])
            .flatten()
            .collect();

        let context = &self.context;

        let source_info_loc = context
            .get_uniform_location(&self.program, "sources")
            .unwrap();

        context.uniform1fv_with_f32_array(Some(&source_info_loc), source_info.as_slice());

        context.draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
    }

    /// Increments all of the Spectrum's sources by one frame.
    pub fn tick(&mut self) {
        self.base.tick();
    }
}

/// Calculates the arctangent, given a quotient in the range [-1, 1].
///
/// Obtained from [IEEE Signal Processing Magazine](http://www-labs.iro.umontreal.ca/~mignotte/IFT2425/Documents/EfficientApproximationArctgFunction.pdf).
///
/// # Parameters
///
/// * `quotient` - the minimum of `cos / sin` and `sin / cos`.
fn atan(quotient: f32) -> f32 {
    (consts::FRAC_PI_4 + 0.273f32 * (1f32 - quotient.abs())) * quotient
}

/// Calculates the arctangent from the cosine and sine.
///
/// # Parameters
///
/// * `cos` - the cosine/x term.
/// * `sin` - the sine/y term.
fn atan2(cos: f32, sin: f32) -> f32 {
    if cos.abs() > sin.abs() {
        if cos < 0f32 {
            atan(sin / cos) + consts::PI
        } else if sin < 0f32 {
            atan(sin / cos) + 2f32 * consts::PI
        } else {
            atan(sin / cos)
        }
    } else if sin < 0f32 {
        -atan(cos / sin) + 3f32 * consts::FRAC_PI_2
    } else {
        -atan(cos / sin) + consts::FRAC_PI_2
    }
}

/// Compiles a WebGL shader from source.
///
/// Obtained from the [`wasm-bindgen` Guide WebGL example](https://rustwasm.github.io/wasm-bindgen/examples/webgl.html).
///
/// # Parameters
///
/// * `context` - the WebGL context.
/// * `shader_type` - the shader's type - vertex or fragment shader.
/// * `source` - the GLSL shader source.
pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> WebGlShader {
    let shader = context.create_shader(shader_type).unwrap();
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap();

    shader
}
