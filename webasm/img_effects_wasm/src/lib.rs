use wasm_bindgen::prelude::*;
use base64::{encode as b64enc, decode as b64dec};
// log_1 functions logs only 1 value
use image::{ load_from_memory, ImageOutputFormat };
use web_sys::console::{log_1 as clog1, log_2 as clog2};

#[wasm_bindgen]
pub fn process_img(b64_content: &str) -> String {
    clog1(&"base64 data recvd".into());
    // lets decode base64 string slice into bytes
    let mut bytes = b64dec(b64_content).unwrap();
    clog1(&"base64 data decoded.".into());

    // retruns DynamicImage type
    let mut img = load_from_memory(&bytes[..]).unwrap();
    clog1(&"bytes to image done.".into());

    // lets apply grayscale filter
    // returns same type here as well
    img = img.grayscale();

    clog1(&"image converted to grayscale.".into());
    
    // lets write image to a buffer as png image
    let mut buf = vec![];
    img.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
    clog1(&"created a png image.".into());

    // encode bytes to base64 back
    let b64_content = b64enc(&buf);
    // return the proper formated base64 string
    // with proper prefix so js takes it as image data
    format!("data:image/png;base64,{}", b64_content)
}
