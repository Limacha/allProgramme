#![cfg(target_arch = "wasm32")]
#![cfg(feature = "web")]
#![allow(non_snake_case)]
use backend::PixelManager::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
pub fn render_to_canvas(canvas_id: &str) {
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::Element = document.get_element_by_id(canvas_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let width: u32 = canvas.width();
    let height: u32 = canvas.height();

    let mut buffer: PixelBuffer = PixelBuffer::new(width, height);
    buffer.FillAll([0, 0, 0, 255]);
    buffer.DrawCenterFullSquare(50, [255, 0, 0, 255]);

    let data: ImageData = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&buffer.pixels),
        width,
        height,
    )
    .unwrap();

    context.put_image_data(&data, 0.0, 0.0).unwrap();
}
