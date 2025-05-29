use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder; // <-- Manquait dans ton code

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Unified Pixels Viewer")
        .with_inner_size(LogicalSize::new(320.0, 240.0))
        .build(&event_loop)
        .expect("Failed to create window");
}
