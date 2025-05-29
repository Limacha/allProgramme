#![allow(non_snake_case)]

#[cfg(not(target_arch = "wasm32"))]
pub fn start() {
    use backend::PixelBuffer;
    use minifb::{Window, WindowOptions};

    let width: usize = 640;
    let height: usize = 360;

    let mut window: Window =
        Window::new("appcrossplat001", width, height, WindowOptions::default()).unwrap();

    let mut pixelBuffer: PixelBuffer = PixelBuffer::new(width, height);

    pixelBuffer.FillAll(150, 0, 100, 255);
    pixelBuffer.draw_center_square(50, 0, 150, 100, 255);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&pixelBuffer.pixels, width, height)
            .unwrap();
    }
}
