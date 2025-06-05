#![cfg(not(target_arch = "wasm32"))]
#![cfg(feature = "native")]
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    monitor::MonitorHandle,
    window::{Window, WindowAttributes},
};

use backend::{Image_manager::*, Pixel_buffer::*};

//task bar buton width 0,01579861
//task bar buton heigth 0,01728395
pub struct App {
    // Option signifie que la valeur peut être présente (Some) ou absente (None).
    ///la fenetre de l'app
    window: Option<Window>,

    ///Gestionnaire du rendu graphique, lié à la fenêtre.
    // Le lifetime 'static garantit que la référence à la fenêtre reste valide aussi longtemps que Pixels existe.
    pixels: Option<Pixels<'static>>,

    ///taille en pixels reels sur l'ecran
    size: PhysicalSize<u32>,

    ///la hauteur du menu
    menuH: u32,

    ///la marge entre les bouttons du menu
    margeXButtonMenu: u32,

    ///le facteur pour logic -> physic
    scaleFactor: f64,

    ///les attributs de la fenetre
    window_attributes: WindowAttributes,

    ///le buffer pour dessiner
    buffer: PixelBuffer,
}

impl App {
    /// Crée une nouvelle instance de `App` avec un titre et une taille de fenêtre spécifiés.
    ///
    /// Initialise les attributs de la fenêtre, mais sans créer la fenêtre ni le rendu.
    ///
    /// * `title` - Le titre de la fenêtre.
    /// * `size` - La taille logique initiale de la fenêtre (en unités indépendantes du DPI).
    ///
    /// -> `Self` - Une instance de `App` avec la configuration initialisée,  
    ///   mais sans fenêtre ni rendu encore créés.
    pub fn new(title: &str, size: LogicalSize<u32>, decorations: bool) -> Self {
        //creez tous les attributs de la fenetre
        let window_attributes: WindowAttributes = Window::default_attributes()
            .with_title(title) //titre de la fenetre
            .with_inner_size(size) //taille de la fenetre en physic
            .with_decorations(decorations); //si on affiche la barre de titre

        let mut buffer: PixelBuffer = PixelBuffer::new(size.width, size.height);

        buffer.FillAll([150, 0, 100, 255]);
        buffer.DrawCenterFullSquare(50, [255, 0, 0, 255]);

        Self {
            window: None,
            pixels: None,
            size: PhysicalSize::new(size.width, size.height),
            menuH: 50,
            margeXButtonMenu: 10,
            scaleFactor: 1.0,
            window_attributes,
            buffer,
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

        let monitor: Option<MonitorHandle> = window.current_monitor();
        match monitor {
            Some(monitor) => {
                self.scaleFactor = monitor.scale_factor();
                self.menuH = (((monitor.size().height as f64 * 0.01728395) * self.scaleFactor)
                    as u32)
                    .max(self.menuH);
                self.margeXButtonMenu = (self.menuH / 5) as u32;
                window.set_min_inner_size(Some(PhysicalSize::new(
                    self.menuH * 4 + self.margeXButtonMenu * 3,
                    self.menuH * 2,
                )));
            }

            None => window.set_min_inner_size(Some(PhysicalSize::new(230, 100))),
        }

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
                            self.buffer.SetSize(self.size.width, self.size.height);
                            self.size = new_size;
                            window.request_redraw();
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        println!(
                            "reaffichagee width:{}, height:{}",
                            self.size.width, self.size.height
                        );
                        self.buffer.SetSize(self.size.width, self.size.height);

                        /* #region menu */
                        //dessine le fond du menu
                        self.buffer.DrawFullRect(
                            0,
                            0,
                            self.size.width,
                            self.menuH,
                            [255, 255, 255, 255],
                        );

                        //dessine l'icon
                        let bufferIcon = ImageManager::ReadIco(
                            "C:\\Users\\Nico\\Documents\\github\\allProgramme\\rust\\appcrosspaltwinit\\frontend\\assets\\img\\mcColors.ico",
                        );
                        match bufferIcon {
                            Ok(Some(bufferIcon)) => {
                                self.buffer.DrawIntoArea(
                                    &bufferIcon,
                                    (self.menuH - (self.menuH - 20)) / 2,
                                    (self.menuH - (self.menuH - 20)) / 2,
                                    self.menuH - 20,
                                    self.menuH - 20,
                                );
                            }
                            _ => println!("no icon found"),
                        }

                        //dessine le tirer du boutton minimize
                        self.buffer.DrawFullRect(
                            self.size.width - (self.menuH * 3) - (self.margeXButtonMenu * 2)
                                + (self.menuH - (self.menuH - 30)) / 2,
                            (self.menuH - 4) / 2,
                            self.menuH - 30,
                            4,
                            [0, 0, 0, 255],
                        );

                        //dessine le carer du boutton full screen
                        self.buffer.DrawBorder(
                            self.size.width - (self.menuH * 2) - (self.margeXButtonMenu)
                                + (self.menuH - (self.menuH - 30)) / 2,
                            (self.menuH - (self.menuH - 30)) / 2,
                            self.menuH - 30,
                            self.menuH - 30,
                            2,
                            2,
                            2,
                            2,
                            [0, 0, 0, 255],
                        );

                        //dessine la croix du boutton fermer
                        self.buffer.DrawCross(
                            self.size.width - (self.menuH) + (self.menuH - (self.menuH - 30)) / 2,
                            (self.menuH - (self.menuH - 30)) / 2,
                            self.menuH - 30,
                            self.menuH - 30,
                            3,
                            [0, 0, 0, 255],
                        );
                        /* #endregion */

                        /*#region placement dans la fenetre*/
                        //buffer mutable des pixel
                        let frame: &mut [u8] = pixels.frame_mut();
                        //copy les pixels sur le rendu
                        frame.copy_from_slice(&self.buffer.pixels);
                        //l'envoie a l'ecran
                        pixels.render().unwrap();
                        /*#endregion*/
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
    width: u32,
    height: u32,
    decorations: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new(title, LogicalSize::new(width, height), decorations);
    event_loop.run_app(&mut app)?;
    Ok(())
}
