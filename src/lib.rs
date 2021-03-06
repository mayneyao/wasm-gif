extern crate cfg_if;
extern crate gif;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use gif::Encoder;
use wasm_bindgen::prelude::*;

mod utils;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub fn frames_array2frames(width: u16, height: u16, frames_array: Vec<u8>) -> Vec<Vec<u8>> {
    let chunk_size = width as usize * height as usize * 4;

    let mut res = Vec::new();
    for v in frames_array.chunks(chunk_size) {
        res.push(rgba2rgb(v))
    }
    res
}

#[wasm_bindgen]
pub fn rgba2rgb(pixels: &[u8]) -> Vec<u8> {
    let mut rgb_pixels: Vec<u8> = Vec::new();
    for v in pixels.chunks(4) {
        rgb_pixels.extend([v[0], v[1], v[2]].iter().cloned())
    }
    rgb_pixels
}

#[wasm_bindgen]
pub fn encode_gif(width: u16, height: u16, frames_array: Vec<u8>) -> Vec<u8> {
    let mut image = Vec::new();
    {
        let mut encoder = Encoder::new(&mut image, width, height, &[]).unwrap();
        for frame in frames_array2frames(width, height, frames_array) {
            let _frame = gif::Frame::from_rgb(width, height, &frame);
            encoder.write_frame(&_frame).unwrap();
        }
    }
    return image;
}
