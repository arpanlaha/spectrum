#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions,
    clippy::upper_case_acronyms
)]

mod spectrum;
mod utils;

pub use spectrum::wasm::SpectrumWasm;
pub use spectrum::webgl::SpectrumWebGL;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
