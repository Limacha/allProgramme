use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        /*let window_attributes = WindowAttributes {
            title: "Ma fenêtre Winit 0.30.11".into(),
            ..Default::default()
        };*/
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
        let window = event_loop.create_window(window_attributes).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            println!("Fermeture demandée.");
            // Ici, vous pouvez implémenter la logique pour fermer proprement l'application.
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App { window: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}
