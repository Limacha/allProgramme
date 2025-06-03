#![allow(non_snake_case)]

/// contient un vecteur avec les pixels a afficher a l'ecran
pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA 4 bytes per pixel
}

impl PixelBuffer {
    /// Crée un nouveau buffer de pixels vierge avec les dimensions données.
    ///
    /// Alloue un vecteur de pixels rempli de zéros (noir transparent),  
    /// chaque pixel étant représenté par 4 octets (RGBA).
    ///
    /// * `width` - Largeur du buffer en pixels.
    /// * `height` - Hauteur du buffer en pixels.
    ///
    /// -> `Self` - Une instance de `PixelBuffer` initialisée avec un fond noir transparent.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    /// Remplit tout le buffer avec une seule couleur RGBA.
    ///
    /// * `r` - Composante rouge (0-255).
    /// * `g` - Composante verte (0-255).
    /// * `b` - Composante bleue (0-255).
    /// * `a` - Composante alpha (0-255).
    pub fn FillAll(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[r, g, b, a]);
        }
    }

    /// Dessine un carré coloré centré dans le buffer avec une taille donnée.
    ///
    /// * `size` - Taille du carré.
    /// * `r` - Composante rouge (0-255).
    /// * `g` - Composante verte (0-255).
    /// * `b` - Composante bleue (0-255).
    /// * `a` - Composante alpha (0-255).
    pub fn DrawCenterSquare(&mut self, size: u32, r: u8, g: u8, b: u8, a: u8) {
        let start_x = (self.width / 2).saturating_sub(size / 2);
        let start_y = (self.height / 2).saturating_sub(size / 2);

        for y in start_y..(start_y + size).min(self.height) {
            for x in start_x..(start_x + size).min(self.width) {
                let i: usize = ((y * self.width + x) * 4) as usize;
                self.pixels[i..i + 4].copy_from_slice(&[r, g, b, a]);
            }
        }
    }

    /// Copie les pixels d’un autre buffer de même type dans ce buffer.
    ///
    /// * `pixelBuffer` - Référence à un `PixelBuffer` source depuis lequel copier les données.
    pub fn CopyFromBuffer(&mut self, pixelBuffer: &PixelBuffer) {
        //va jusqua la plus petite taille en hauteur
        for y in 0..self.height.min(pixelBuffer.height) {
            //va jusqua la plus petite taille en largeur
            for x in 0..self.width.min(pixelBuffer.width) {
                let iFrom: usize = ((y * pixelBuffer.width + x) * 4) as usize; //prend la position du pixel dans le buffer fournit
                let iTo: usize = ((y * self.width + x) * 4) as usize; //prend la position du pixels dans le buffer cible
                self.pixels[iTo..iTo + 4].copy_from_slice(&pixelBuffer.pixels[iFrom..iFrom + 4]); //copy les valeur du fournit dans le cible
            }
        }
    }

    /// Copie les pixels d’un vecteur externe dans ce buffer.
    ///
    /// * `pixels` - Vecteur de pixels source (format RGBA, 4 octets par pixel).
    /// * `width` - Largeur du buffer source.
    /// * `height` - Hauteur du buffer source.
    pub fn CopyFromVecteur(&mut self, pixels: Vec<u8>, width: u32, height: u32) {
        //va jusqua la plus petite taille en hauteur
        for y in 0..self.height.min(height) {
            //va jusqua la plus petite taille en largeur
            for x in 0..self.width.min(width) {
                let iFrom: usize = ((y * width + x) * 4) as usize; //prend la position du pixel dans le buffer fournit
                let iTo: usize = ((y * self.width + x) * 4) as usize; //prend la position du pixels dans le buffer cible
                self.pixels[iTo..iTo + 4].copy_from_slice(&pixels[iFrom..iFrom + 4]); //copy les valeur du fournit dans le cible
            }
        }
    }

    /// Redimensionne le buffer tout en conservant le contenu possible de l’ancien.
    ///
    /// * `width` - Nouvelle largeur en pixels.
    /// * `height` - Nouvelle hauteur en pixels.
    pub fn SetSize(&mut self, width: u32, height: u32) {
        let mut newBuffer: PixelBuffer = PixelBuffer::new(width, height);
        newBuffer.CopyFromBuffer(self);
        self.width = newBuffer.width;
        self.height = newBuffer.height;
        self.pixels = newBuffer.pixels;
    }
}
