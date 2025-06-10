use std::fs::File;
use std::io::{BufWriter, Write};
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

    pub fn save_buffer_summary(&self, path: &str) -> std::io::Result<()> {
        let file: File = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        writeln!(writer, "PixelBuffer: {}x{}", self.width, self.height)?;
        writeln!(writer, "Total pixels: {}", self.pixels.len() / 4)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let i: usize = ((y * self.width + x) * 4) as usize;
                let r: u8 = self.pixels[i];
                let g: u8 = self.pixels[i + 1];
                let b: u8 = self.pixels[i + 2];
                let a: u8 = self.pixels[i + 3];
                writeln!(writer, "({},{}): RGBA({}, {}, {}, {})", x, y, r, g, b, a)?;
            }
        }

        Ok(())
    }

    pub fn save_buffer_brut(&self, path: &str) -> std::io::Result<()> {
        let file: File = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        writeln!(writer, "PixelBuffer: {}x{}", self.width, self.height)?;
        writeln!(writer, "Total pixels: {}", self.pixels.len() / 4)?;

        write!(writer, "[")?;
        for y in 0..self.height {
            for x in 0..self.width {
                let i: usize = ((y * self.width + x) * 4) as usize;
                let r: u8 = self.pixels[i];
                let g: u8 = self.pixels[i + 1];
                let b: u8 = self.pixels[i + 2];
                let a: u8 = self.pixels[i + 3];
                write!(writer, "({}, {}, {}, {})", r, g, b, a)?;
            }
            writeln!(writer, " ")?;
        }
        writeln!(writer, "]")?;

        Ok(())
    }

    /// Sauvegarde une image RGBA dans un fichier BMP sans crate externe.
    pub fn write_buffer_to_bmp(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Constantes
        let pixel_size = 4;
        let padding_per_row = (4 - (self.width * pixel_size) % 4) % 4;
        let row_size = self.width * pixel_size + padding_per_row;
        let pixel_data_size = row_size * self.height;
        let file_size = 14 + 40 + pixel_data_size;

        // --- BMP HEADER (14 octets) ---
        writer.write_all(b"BM")?; // Signature
        writer.write_all(&(file_size as u32).to_le_bytes())?; // File size
        writer.write_all(&[0u8; 4])?; // Reserved
        writer.write_all(&(14u32 + 40u32).to_le_bytes())?; // Offset to pixel data

        // --- DIB HEADER (40 octets) ---
        writer.write_all(&40u32.to_le_bytes())?; // Header size
        writer.write_all(&(self.width as i32).to_le_bytes())?; // Width
        writer.write_all(&(self.height as i32).to_le_bytes())?; // Height (positive = bottom-up)
        writer.write_all(&1u16.to_le_bytes())?; // Planes
        writer.write_all(&32u16.to_le_bytes())?; // Bits per pixel
        writer.write_all(&0u32.to_le_bytes())?; // Compression = BI_RGB
        writer.write_all(&(pixel_data_size as u32).to_le_bytes())?; // Image size
        writer.write_all(&[0u8; 16])?; // Resolution & palette (unused)

        // --- Pixel Data (bottom-up, BGRA) ---
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let i = ((y * self.width + x) * 4) as usize;
                let r = self.pixels[i];
                let g = self.pixels[i + 1];
                let b = self.pixels[i + 2];
                let a = self.pixels[i + 3]; // Ignoré ou utilisé dans BGRA

                // BMP attend BGRA
                writer.write_all(&[b, g, r, a])?;
            }
            writer.write_all(&vec![0u8; padding_per_row as usize])?;
        }

        Ok(())
    }

    pub fn DrawIntoArea(
        &mut self,
        src: &PixelBuffer,
        dst_x: u32,
        dst_y: u32,
        target_width: u32,
        target_height: u32,
    ) {
        let mut scaled = PixelBuffer::new(target_width, target_height);
        scaled.CopyScaledDown(src);
        self.CopyRegionTo(&scaled, dst_x, dst_y, target_width, target_height);
    }

    /// Copie une région depuis un `PixelBuffer` source vers ce buffer (destination).
    ///
    /// # Paramètres
    /// * `src` - Le buffer source à copier.
    /// * `dst_x` - Coordonnée X du coin supérieur gauche de la zone de destination.
    /// * `dst_y` - Coordonnée Y du coin supérieur gauche de la zone de destination.
    /// * `dst_width` - Largeur maximale à copier dans la destination.
    /// * `dst_height` - Hauteur maximale à copier dans la destination.
    pub fn CopyRegionTo(
        &mut self,
        src: &PixelBuffer,
        dst_x: u32,
        dst_y: u32,
        dst_width: u32,
        dst_height: u32,
    ) {
        // Calcule la largeur à copier en prenant la plus petite entre :
        // - la largeur de la source
        // - la largeur demandée
        // - l'espace restant dans la destination depuis `dst_x`
        let copy_width = src
            .width
            .min(dst_width)
            .min(self.width.saturating_sub(dst_x));

        // Calcule la hauteur à copier en prenant la plus petite entre :
        // - la hauteur de la source
        // - la hauteur demandée
        // - l'espace restant dans la destination depuis `dst_y`
        let copy_height = src
            .height
            .min(dst_height)
            .min(self.height.saturating_sub(dst_y));

        // Parcours chaque ligne de la zone à copier
        for y in 0..copy_height {
            // Parcours chaque colonne de la ligne
            for x in 0..copy_width {
                // Calcule l'index dans le buffer source
                let src_i: usize = ((y * src.width + x) * 4) as usize;

                // Calcule l'index dans le buffer destination
                let dst_i: usize = (((dst_y + y) * self.width + (dst_x + x)) * 4) as usize;

                // Copie les 4 octets (RGBA) du pixel depuis la source vers la destination
                self.pixels[dst_i..dst_i + 4].copy_from_slice(&src.pixels[src_i..src_i + 4]);
            }
        }
    }

    /// Réduit la taille d'un buffer source pour l'adapter à ce buffer en effectuant une copie.
    ///
    /// * `src` - Le buffer source à redimensionner et copier.
    pub fn CopyScaledDown(&mut self, src: &PixelBuffer) {
        // Calcule le facteur de réduction horizontal
        let step_x = src.width as f32 / self.width as f32;
        // Calcule le facteur de réduction vertical
        let step_y = src.height as f32 / self.height as f32;

        // Parcourt chaque pixel destination
        for y in 0..self.height {
            for x in 0..self.width {
                // Calcule la position correspondante dans le buffer source (en prenant le pixel supérieur-gauche)
                let src_x = (x as f32 * step_x).floor() as u32;
                let src_y = (y as f32 * step_y).floor() as u32;

                // Calcule l'indice du pixel source
                let src_index = ((src_y * src.width + src_x) * 4) as usize;
                // Calcule l'indice du pixel destination
                let dst_index = ((y * self.width + x) * 4) as usize;

                // Copie le pixel RGBA du buffer source vers le pixel réduit
                self.pixels[dst_index..dst_index + 4]
                    .copy_from_slice(&src.pixels[src_index..src_index + 4]);
            }
        }
    }

    /// Remplit tout le buffer avec une seule couleur RGBA.
    ///
    /// * `color` - La couleur du rectangle au format RGBA (4 octets).
    pub fn FillAll(&mut self, color: [u8; 4]) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&color);
        }
    }

    /// Dessine un carré coloré centré dans le buffer avec une taille donnée.
    ///
    /// * `size` - Taille du carré.
    /// * `color` - La couleur du rectangle au format RGBA (4 octets).
    /// * `fill` - Si `true`, dessine un rectangle rempli, sinon dessine uniquement le contour.
    pub fn DrawCenterFullSquare(&mut self, size: u32, color: [u8; 4]) {
        self.DrawFullRect(
            (self.width / 2).saturating_sub(size / 2),
            (self.height / 2).saturating_sub(size / 2),
            size,
            size,
            color,
        );
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

    /// Dessine un rectangle sur le buffer de pixels.
    ///
    /// * `startX` - La coordonnée X du coin supérieur gauche du rectangle.
    /// * `startY` - La coordonnée Y du coin supérieur gauche du rectangle.
    /// * `width` - La largeur du rectangle en pixels.
    /// * `height` - La hauteur du rectangle en pixels.
    /// * `color` - La couleur du rectangle au format RGBA (4 octets).
    /// * `fill` - Si `true`, dessine un rectangle rempli, sinon dessine uniquement le contour.
    pub fn DrawFullRect(
        &mut self,
        startX: u32,
        startY: u32,
        width: u32,
        height: u32,
        color: [u8; 4],
    ) {
        let max_x: u32 = (startX + width).min(self.width);
        let max_y: u32 = (startY + height).min(self.height);

        // Remplissage complet : dessiner tous les pixels à l'intérieur du rectangle
        for y in startY..max_y {
            for x in startX..max_x {
                let i: usize = ((y * self.width + x) * 4) as usize;
                self.pixels[i..i + 4].copy_from_slice(&color);
            }
        }
    }

    /// Dessine un contour autour d'un rectangle sur le buffer de pixels.
    ///
    /// * `startX`, `startY` : Coordonnées du coin supérieur gauche.
    /// * `width`, `height` : Dimensions du rectangle.
    /// * `thickness_*` : Épaisseurs des bords (gauche, droit, haut, bas).
    /// * `color` : Couleur au format `[R, G, B, A]`.
    pub fn DrawBorder(
        &mut self,
        startX: u32,
        startY: u32,
        width: u32,
        height: u32,
        thickness_left: u32,
        thickness_right: u32,
        thickness_top: u32,
        thickness_bottom: u32,
        color: [u8; 4],
    ) {
        let end_x: u32 = (startX + width).min(self.width); //dernier point dispo en x
        let end_y: u32 = (startY + height).min(self.height); //dernier point dispo en y

        let thick_left: u32 = thickness_left.min(width); //epaiseur pas plus grand que la largeur
        let thick_right: u32 = thickness_right.min(width); //epaiseur pas plus grand que la largeur
        let thick_top: u32 = thickness_top.min(height); //epaiseur pas plus grand que la hauteur
        let thick_bottom: u32 = thickness_bottom.min(height); //epaiseur pas plus grand que la hauteur

        // Bords horizontaux
        //parcour toute la largeur
        for x in startX..end_x {
            //parcour l'epaiseur voulut haut
            for y in 0..thick_top {
                //position du pixel
                let i: usize = (((startY + y) * self.width + x) * 4) as usize;
                self.pixels[i..i + 4].copy_from_slice(&color);
            }
            //le cote haut ne prend pas tout
            if (thick_top < self.height) {
                //parcour l'epaiseur voulut bas
                for y in 0..thick_bottom {
                    //position du pixel
                    let i: usize = (((end_y - 1 - y) * self.width + x) * 4) as usize;
                    self.pixels[i..i + 4].copy_from_slice(&color);
                }
            }
        }

        // Bords verticaux : gauche et droite
        for y in startY..end_y {
            for x in 0..thick_left {
                //position du pixel
                let i: usize = ((y * self.width + startX + x) * 4) as usize;
                self.pixels[i..i + 4].copy_from_slice(&color);
            }
            //si le coter gauche ne prend pas tout
            if (thick_left < self.width) {
                for x in 0..thick_right {
                    //position du pixel
                    let i: usize = ((y * self.width - 1 + end_x - x) * 4) as usize;
                    self.pixels[i..i + 4].copy_from_slice(&color);
                }
            }
        }
    }

    /// Dessine une croix (X) en partant du coin supérieur gauche (startX, startY).
    /// La croix est composée de deux diagonales, avec une certaine épaisseur.
    ///
    /// - `startX`, `startY` : Position de départ (coin supérieur gauche).
    /// - `width`, `height` : Dimensions totales de la croix.
    /// - `thickness` : Épaisseur des diagonales.
    /// - `color` : Couleur de la croix (RGBA).
    pub fn DrawCross(
        &mut self,
        startX: u32,
        startY: u32,
        width: u32,
        height: u32,
        thickness: u32,
        color: [u8; 4],
    ) {
        for y in 0..height {
            for x in 0..width {
                //i32 pour les - et val abs
                let rel_x: i32 = x as i32;
                let rel_y: i32 = y as i32;

                // Condition pour la première diagonale \
                let diag1: bool = (rel_x - rel_y).abs() < thickness as i32;
                // Condition pour la deuxième diagonale /
                let diag2: bool = (rel_x + rel_y - height as i32).abs() < thickness as i32;

                if diag1 || diag2 {
                    //prend la position du pixels
                    let px: u32 = startX + x;
                    let py: u32 = startY + y;

                    // Vérifie que le pixel est bien dans les limites de l'écran
                    if px < self.width && py < self.height {
                        let i: usize = ((py * self.width + px) * 4) as usize;
                        self.pixels[i..i + 4].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}
