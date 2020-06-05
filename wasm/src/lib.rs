use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

mod utils;

use js_sys::Math;
use std::f32::consts;
use std::iter;

const TWO_PI: f32 = consts::PI * 2f32;
const TWO_THIRDS_PI: f32 = consts::FRAC_PI_3 * 2f32;
const FOUR_THIRDS_PI: f32 = consts::FRAC_PI_3 * 4f32;
const FIVE_THIRDS_PI: f32 = consts::FRAC_PI_3 * 5f32;
const DH_UPPER: f32 = consts::FRAC_PI_3 / 10f32;
const DH_HALF: f32 = DH_UPPER / 2f32;
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
        if self.0 >= TWO_PI {
            self.0 -= TWO_PI;
        } else if self.0 <= 0f32 {
            self.0 += TWO_PI;
        }
    }

    fn to_rgba(self) -> RGBA {
        let hue = self.0;
        let primary = 255;
        if hue < consts::PI {
            if hue < consts::FRAC_PI_3 {
                RGBA::from_rgb(primary, (255f32 * hue / consts::FRAC_PI_3) as u8, 0)
            } else if hue < TWO_THIRDS_PI {
                RGBA::from_rgb(
                    (255f32 * (2f32 - hue / consts::FRAC_PI_3)) as u8,
                    primary,
                    0,
                )
            } else {
                RGBA::from_rgb(
                    0,
                    primary,
                    (255f32 * (hue / consts::FRAC_PI_3 - 2f32)) as u8,
                )
            }
        } else if hue < FOUR_THIRDS_PI {
            RGBA::from_rgb(
                0,
                (255f32 * (4f32 - hue / consts::FRAC_PI_3)) as u8,
                primary,
            )
        } else if hue < FIVE_THIRDS_PI {
            RGBA::from_rgb(
                (255f32 * (hue / consts::FRAC_PI_3 - 4f32)) as u8,
                0,
                primary,
            )
        } else {
            RGBA::from_rgb(
                primary,
                0,
                (255f32 * (6f32 - hue / consts::FRAC_PI_3)) as u8,
            )
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
    hue_sin: f32,
    hue_cos: f32,
    hue_vectors: (f32, f32),
}

impl Source {
    pub fn new(width: f32, height: f32) -> Source {
        let x = Math::random() as f32 * width;
        let y = Math::random() as f32 * height;
        let dx = Math::random() as f32 * 2f32 - 1f32;
        let dy = Math::random() as f32 * 2f32 - 1f32;
        let dh = Math::random() as f32 * DH_UPPER - DH_HALF;
        let hue = Hue(Math::random() as f32 * TWO_PI);

        let hue_val = hue.get();

        let hue_cos = hue_val.cos();
        let hue_sin = hue_val.sin();
        Source {
            x,
            y,
            dx,
            dy,
            dh,
            hue,
            hue_vectors: (hue_cos, hue_sin),
            hue_cos,
            hue_sin,
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

    pub fn hue_cos(&self) -> f32 {
        self.hue_cos
    }

    pub fn hue_sin(&self) -> f32 {
        self.hue_sin
    }

    pub fn tick(&mut self, width: f32, height: f32) {
        self.hue.tick(self.dh);
        let hue_rad = self.hue.get();
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
pub struct Spectrum {
    width: usize,
    height: usize,
    sources: Vec<Source>,
    data: Vec<RGBA>,
    context: WebGlRenderingContext,
    program: WebGlProgram,
}

#[wasm_bindgen]
impl Spectrum {
    pub fn new(
        width: usize,
        height: usize,
        num_sources: usize,
        context: WebGlRenderingContext,
    ) -> Spectrum {
        utils::set_panic_hook();
        let width_float = width as f32;
        let height_float = height as f32;
        let sources: Vec<Source> = iter::repeat(())
            .map(|()| Source::new(width_float, height_float))
            .take(num_sources)
            .collect();

        // let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document.get_element_by_id("spectrum-canvas").unwrap();
        // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        // let context = canvas
        //     .get_context("webgl")?
        //     .unwrap()
        //     .dyn_into::<WebGlRenderingContext>()?;

        let vertex_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
                attribute vec4 a_position;

                void main(void) {
                    gl_Position = a_position;
                }
            "#,
        )
        .unwrap();

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

                    precision mediump float;

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
        ).unwrap();

        let program = link_program(&context, &vertex_shader, &fragment_shader).unwrap();
        context.use_program(Some(&program));

        let position_attribute_loc = context.get_attrib_location(&program, "a_position");

        assert!(
            position_attribute_loc >= 0,
            "a_position: {}",
            position_attribute_loc
        );
        let position_attribute_loc = position_attribute_loc as u32;

        let vertex_coords = [-1f32, -1f32, 1f32, -1f32, 1f32, 1f32, -1f32, 1f32];

        let buffer = context
            .create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
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

        let spectrum = Spectrum {
            width,
            height,
            sources,
            data: vec![RGBA::from_rgb(0, 0, 0); width * height],
            context,
            program,
        };
        spectrum.draw();

        spectrum
    }

    pub fn draw(&self) {
        // utils::set_panic_hook();

        // for x in 0..self.width {
        //     let x_float = x as f32;
        //     for y in 0..self.height {
        //         let y_float = y as f32;
        //         let (mut hue_vector_cos, mut hue_vector_sin) = (0f32, 0f32);
        //         for source in &self.sources {
        //             let (source_vector_cos, source_vector_sin) = source.hue_vectors();
        //             let dist_factor =
        //                 (x_float - source.x()).powi(2) + (y_float - source.y()).powi(2) + 1f32;
        //             hue_vector_cos += source_vector_cos / dist_factor;
        //             hue_vector_sin += source_vector_sin / dist_factor;
        //         }

        //         self.data[x + y * self.width] =
        //             Hue(atan2(hue_vector_cos, hue_vector_sin)).to_rgba();
        //     }
        // }
        let source_info: Vec<f32> = self
            .sources
            .iter()
            .map(|source| vec![source.x(), source.y(), source.hue_cos(), source.hue_sin()])
            .flatten()
            .collect();

        let source_info = source_info.as_slice();

        // unsafe {
        // let source_info = js_sys::Float32Array::view(source_info);

        let source_info_loc = self
            .context
            .get_uniform_location(&self.program, "sources")
            .unwrap();

        self.context
            .uniform1fv_with_f32_array(Some(&source_info_loc), source_info);

        // self.context.buffer_data_with_array_buffer_view(
        //     WebGlRenderingContext::ARRAY_BUFFER,
        //     &source_info,
        //     WebGlRenderingContext::DYNAMIC_DRAW,
        // );
        // }

        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLE_FAN, 0, 4);
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

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
