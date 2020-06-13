use std::f32::consts;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, ImageData, WebGlProgram, WebGlRenderingContext,
    WebGlShader,
};

mod base;

use base::{BaseSpectrum, Hue, RGBA};

const TWO_PI: f32 = consts::PI * 2f32;
const THREE_HALVES_PI: f32 = consts::PI * 1.5f32;

/// A WebAssembly-only implementation of Spectrum.
#[wasm_bindgen]
pub struct SpectrumWasm {
    /// The Spectrum's BaseSpectrum.
    base: BaseSpectrum,

    /// The Spectrum's pixel data.
    data: Vec<u8>,

    /// The `2d` context belonging to the Spectrum's canvas.
    context: CanvasRenderingContext2d,
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
    pub fn new(
        width: usize,
        height: usize,
        num_sources: usize,
        movement_speed: f32,
        color_speed: f32,
        canvas: HtmlCanvasElement,
    ) -> SpectrumWasm {
        let mut spectrum = SpectrumWasm {
            base: BaseSpectrum::new(width, height, num_sources, movement_speed, color_speed),
            data: vec![0u8; width * height * 4],
            context: canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap(),
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
        for x in 0..self.base.width() {
            let x_float = x as f32;
            for y in 0..self.base.height() {
                let (hue_vector_cos, hue_vector_sin) =
                    self.base
                        .sources()
                        .iter()
                        .fold((0f32, 0f32), |(sum_cos, sum_sin), source| {
                            let dist_factor = (x_float - source.x()).powi(2)
                                + (y as f32 - source.y()).powi(2)
                                + 1f32;
                            (
                                sum_cos + source.hue_cos() / dist_factor,
                                sum_sin + source.hue_sin() / dist_factor,
                            )
                        });

                let RGBA(r, g, b, a) =
                    Hue::new(atan2_approx(hue_vector_cos, hue_vector_sin)).to_rgba();

                let start = (x + y * self.base.width()) * 4;

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
                    self.base.width() as u32,
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
        canvas: HtmlCanvasElement,
    ) -> SpectrumGL {
        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

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
                    #define TWO_PI 6.283185307179586
                    #define PI_4 0.7853981633974483
                    #define PI_3 1.0471975511965976
                    #define PI_2 1.5707963267948966
                    #define PI_3_2 4.71238898038469
                    #define PI_2_3 2.0943951023931953
                    #define PI_4_3 4.1887902047863905
                    #define PI_5_3 5.235987755982989

                    precision highp float;

                    uniform float sources[{}];

                    float atan_approx(float quotient) {{
                        return (PI_4 + 0.273 * (1.0 - abs(quotient))) * quotient;
                    }}

                    float atan2_approx(float x, float y) {{
                        if (abs(x) > abs(y)) {{
                            if (x < 0.0) {{
                                return atan_approx(y / x) + PI;
                            }} else if (y < 0.0) {{
                                return atan_approx(y / x) + TWO_PI;
                            }} else {{
                                return atan_approx(y / x);
                            }}
                        }} else if (y < 0.0) {{
                            return PI_3_2 - atan_approx(x / y);
                        }} else {{
                            return PI_2 - atan_approx(x / y);
                        }}
                    }}

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

                        float hue = atan2_approx(sin_sum, cos_sum);
                       
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
            .sources()
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
fn atan_approx(quotient: f32) -> f32 {
    (consts::FRAC_PI_4 + 0.273f32 * (1f32 - quotient.abs())) * quotient
}

/// Calculates the arctangent from the cosine and sine.
///
/// # Parameters
///
/// * `cos` - the cosine/x term.
/// * `sin` - the sine/y term.
fn atan2_approx(cos: f32, sin: f32) -> f32 {
    if cos.abs() > sin.abs() {
        if cos < 0f32 {
            atan_approx(sin / cos) + consts::PI
        } else if sin < 0f32 {
            atan_approx(sin / cos) + TWO_PI
        } else {
            atan_approx(sin / cos)
        }
    } else if sin < 0f32 {
        -atan_approx(cos / sin) + THREE_HALVES_PI
    } else {
        -atan_approx(cos / sin) + consts::FRAC_PI_2
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
