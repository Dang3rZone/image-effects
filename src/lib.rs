use base64::decode;
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Imagae decoded".into());

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    image = image.grayscale();
    log(&"Image grayscaled".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"Image written to buffer".into());
}
