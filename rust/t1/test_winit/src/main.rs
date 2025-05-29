use minifb::{Window, WindowOptions};

fn main() {
    let mut window = Window::new("Test Window", 640, 360, WindowOptions::default()).unwrap();

    let mut buffer: Vec<u32> = vec![0; 640 * 360];

    // Exemple : mettre un pixel rouge au centre
    buffer[1] = 0xFF0000; // Rouge en ARGB
    buffer[2] = 0xFF0000; // Rouge en ARGB
    buffer[3] = 0xFF0000; // Rouge en ARGB
    buffer[4] = 0xFF0000; // Rouge en ARGB
    buffer[5] = 0xFF0000; // Rouge en ARGB
    buffer[6] = 0xFF0000; // Rouge en ARGB

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, 640, 360).unwrap();
    }
}
