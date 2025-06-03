#![allow(non_snake_case)]

#[cfg(target_arch = "wasm32")]
use backend::PixelBuffer;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn render_to_canvas(canvas_id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let width = canvas.width();
    let height = canvas.height();

    let mut buffer = PixelBuffer::new(width, height);
    buffer.clear(0, 0, 0, 255);
    buffer.draw_center_square(50, 255, 0, 0, 255);

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&buffer.pixels),
        width,
        height,
    )
    .unwrap();

    context.put_image_data(&data, 0.0, 0.0).unwrap();
}
