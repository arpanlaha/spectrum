mod spectrum;
mod utils;

pub use spectrum::wasm::SpectrumWasm;
pub use spectrum::webgl::SpectrumWebGL;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
