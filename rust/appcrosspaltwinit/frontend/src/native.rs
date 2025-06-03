use backend::PixelBuffer;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

pub struct App {
    // Option signifie que la valeur peut être présente (Some) ou absente (None).
    window: Option<Window>,              //la fenetre en question
    pixels: Option<Pixels<'static>>, // Gestionnaire du rendu graphique, lié à la fenêtre. Le lifetime 'static garantit que la référence à la fenêtre reste valide aussi longtemps que Pixels existe.
    size: PhysicalSize<u32>,         //taile en pixels reels sur l'ecran
    window_attributes: WindowAttributes, //les attributs de la fenetre
}

impl App {
    /// Crée une nouvelle instance de `App` avec un titre et une taille de fenêtre spécifiés.
    ///
    /// Initialise les attributs de la fenêtre, mais sans créer la fenêtre ni le rendu.
    ///
    /// # Arguments
    ///
    /// * `title` - Le titre de la fenêtre.
    /// * `size` - La taille logique initiale de la fenêtre (en unités indépendantes du DPI).
    ///
    /// # Retour
    ///
    /// * `Self` - Une instance de `App` avec la configuration initialisée,  
    ///   mais sans fenêtre ni rendu encore créés.
    pub fn new(title: &str, size: LogicalSize<f64>, decorations: bool) -> Self {
        //creez tous les attributs de la fenetre
        let window_attributes: WindowAttributes = Window::default_attributes()
            .with_title(title) //titre de la fenetre
            .with_inner_size(size) //taille de la fenetre en physic
            .with_decorations(decorations); //si on affiche la barre de titre

        Self {
            window: None,                      //pas de fenetre creez
            pixels: None,                      // L'interface de rendu n'est pas encore initialisée.
            size: PhysicalSize::new(128, 128), //la taille physique de la fenetre
            window_attributes,                 //les attributs de la fenetre
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed");
        let window: Window = event_loop
            .create_window(self.window_attributes.clone())
            .unwrap();
        self.size = window.inner_size();

        // SAFETY: We extend the window's lifetime to 'static because it will live
        // as long as the app does. This is necessary for Pixels<'static>.
        let static_window: &'static Window = unsafe { std::mem::transmute(&window) };

        let surface_texture: SurfaceTexture<&'static Window> =
            SurfaceTexture::new(self.size.width, self.size.height, static_window); //creation de la surface de rendu (ou affiche les pixels)
        let pixels = Pixels::new(self.size.width, self.size.height, surface_texture).unwrap(); //instancie la class pixel avec un frame(tableau des pixels) et la surface de texture

        self.pixels = Some(pixels);
        self.window = Some(window); //stock le fenetre

        self.window.as_ref().unwrap().request_redraw(); //prend un reference a la fenetre pour pouvoir appeller une fonction -> redraw
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        //si un des some est null et n'est pas un type voulut alors false sinon on utilise
        if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
            if window.id() == window_id {
                //verifie que s'est la bonne fenetre
                match event {
                    WindowEvent::CloseRequested => event_loop.exit(), //ferme la fenetre
                    WindowEvent::Resized(new_size) => {
                        if let Err(e) = pixels.resize_surface(new_size.width, new_size.height) {
                            eprintln!("Erreur lors du redimensionnement de la surface : {e}");
                        } else if let Err(e) = pixels.resize_buffer(new_size.width, new_size.height)
                        {
                            eprintln!("Erreur lors du redimensionnement du buffer : {e}");
                        } else {
                            self.size = new_size;
                            window.request_redraw();
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        println!(
                            "reaffichage width:{}, height{}",
                            self.size.width, self.size.height
                        );
                        let frame: &mut [u8] = pixels.frame_mut(); //buffer mutable des pixel

                        let mut buffer: PixelBuffer =
                            PixelBuffer::new(self.size.width, self.size.height); //cree un nouveau buffer
                        buffer.pixels.fill(0); //remplit de noir
                        buffer.draw_center_square(50, 255, 0, 0, 255); //dessine un carre

                        frame.copy_from_slice(&buffer.pixels); //copy les pixels sur le rendu
                        pixels.render().unwrap(); //l'envoie a l'ecran
                    }
                    _ => {
                        println!("Autre événement reçu : {:?}", event);
                    }
                }
            }
        }
    }
}
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
*/
/// Lance une nouvelle application graphique avec un titre et une taille de fenêtre spécifiés.
///
/// # Arguments
///
/// * `title` - Le titre de la fenêtre.
/// * `width` - La largeur logique initiale de la fenêtre (en unités indépendantes du DPI).
/// * `height` - La hauteur logique initiale de la fenêtre (en unités indépendantes du DPI).
///
/// # Retour
///
/// * `Result<(), Box<dyn std::error::Error>>` -  
///   Renvoie `Ok(())` si l'application s'est lancée correctement,  
///   sinon une erreur encapsulée dans un `Box`.
pub fn launch_new_app(
    title: &str,
    width: f64,
    height: f64,
    decorations: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(title, LogicalSize::new(width, height), decorations);
    event_loop.run_app(&mut app)?;
    Ok(())
}
