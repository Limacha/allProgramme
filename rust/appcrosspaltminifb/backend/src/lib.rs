#![allow(non_snake_case)]

/// contient un vecteur avec les pixels a afficher a l'ecran
pub struct PixelBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>, // RGBA 4 bytes per pixel tous sur une ligne pas de 2D
}

impl PixelBuffer {
    /// Créé un nouveau buffer pixel vierge
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height], //creez le tableau en fonction de la largeur, de la hauteur et multiplie par 4 pour le rgba
        }
    }

    /// passe de valeur rgba a un u32 pour minifb
    ///
    /// # Arguments
    /// - `r`: composante rouge (0-255)
    /// - `g`: composante verte (0-255)
    /// - `b`: composante bleue (0-255)
    /// - `a`: composante alpha (opacité, 0-255)
    pub fn RGBAToMinifbU32(r: u8, g: u8, b: u8, a: u8) -> u32 {
        return ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    }

    /// Remplit tout le buffer de pixels avec une couleur uniforme RGBA.
    ///
    /// # Arguments
    /// - `r`: composante rouge (0-255)
    /// - `g`: composante verte (0-255)
    /// - `b`: composante bleue (0-255)
    /// - `a`: composante alpha (opacité, 0-255)
    pub fn FillAll(&mut self, r: u8, g: u8, b: u8, a: u8) {
        let pixel: u32 = Self::RGBAToMinifbU32(r, g, b, a);
        //prend chaque valeur du vec comme reference mutable en gros fait reference a chaque valeur
        for p in self.pixels.iter_mut() {
            //* pour modifier la valeur dans le vec et pas le pointeur
            *p = pixel;
        }
    }

    /// Dessine un carré plein coloré au centre du buffer
    ///
    /// # Arguments
    /// - `size`: taille du carre
    /// - `r`: composante rouge (0-255)
    /// - `g`: composante verte (0-255)
    /// - `b`: composante bleue (0-255)
    /// - `a`: composante alpha (opacité, 0-255)
    pub fn draw_center_square(&mut self, size: usize, r: u8, g: u8, b: u8, a: u8) {
        //calcul la position de depard si underflow -> return 0
        let start_x: usize = (self.width / 2).saturating_sub(size / 2);
        let start_y: usize = (self.height / 2).saturating_sub(size / 2);

        //parcourt chaque position sans sortir du cadre
        for y in start_y..(start_y + size).min(self.height) {
            for x in start_x..(start_x + size).min(self.width) {
                //calcul le debut du pixel dans le buffer
                let i: usize = y * self.width + x;
                //defini la couleur du pixel
                self.pixels[i] = Self::RGBAToMinifbU32(r, g, b, a);
            }
        }
    }
}
