/// contient un vecteur avec les pixels a afficher a l'ecran
pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA 4 bytes per pixel
}

impl PixelBuffer {
    /// Créé un nouveau buffer pixel vierge
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    /// Remplit le buffer avec une couleur unie RGBA
    pub fn fill_all(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[r, g, b, a]);
        }
    }

    /// Dessine un carré coloré au centre du buffer
    pub fn draw_center_square(&mut self, size: u32, r: u8, g: u8, b: u8, a: u8) {
        let start_x = (self.width / 2).saturating_sub(size / 2);
        let start_y = (self.height / 2).saturating_sub(size / 2);

        for y in start_y..(start_y + size).min(self.height) {
            for x in start_x..(start_x + size).min(self.width) {
                let i = ((y * self.width + x) * 4) as usize;
                self.pixels[i..i + 4].copy_from_slice(&[r, g, b, a]);
            }
        }
    }
}
