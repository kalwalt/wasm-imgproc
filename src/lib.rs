mod utils;

use wasm_bindgen::prelude::*;
use imageproc::utils::gray_bench_image;
use imageproc::filter::gaussian_blur_f32;
use imageproc::noise::salt_and_pepper_noise;
use image::{DynamicImage};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn image_create(width: u32, height: u32) -> Vec<u8> {
    utils::set_panic_hook();
    let image = gray_bench_image(width, height);
    let noised_image = salt_and_pepper_noise(&image, 0.3, 14);
    let sigma = 0.8;
    let filtered_image = gaussian_blur_f32(&noised_image, sigma);
    let dyn_image = DynamicImage::ImageLuma8(filtered_image).into_rgba8();
    let out = dyn_image.into_vec();
    out
}
