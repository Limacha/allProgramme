use crate::Pixel_buffer::PixelBuffer;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct ImageManager {}
impl ImageManager {
    /// Lit une icône ICO depuis un chemin en dur et retourne un PixelBuffer si possible.
    ///
    /// Cette fonction supporte uniquement les images PNG ou BMP 32 bits non compressées incluses dans l'ICO.
    /// - Pour les PNG, elle détecte le format mais ne les décode pas.
    /// - Pour les BMP, elle décode manuellement l'image en supposant un format 32 bits.
    /// Retourne `Ok(Some(PixelBuffer))` si l'image est décodée avec succès, sinon `Ok(None)` ou une erreur.
    pub fn ReadIco(path: &str) -> std::io::Result<Option<PixelBuffer>> {
        // Ouvre le fichier ICO à un chemin spécifique
        let mut file = File::open(path)?;

        // Lire les 6 octets d'en-tête ICO (Reserved, Type, Count)
        let mut header = [0u8; 6];
        file.read_exact(&mut header)?;

        // Nombre d'images contenues dans l'ICO
        let count = u16::from_le_bytes([header[4], header[5]]);
        println!("Nombre d'images: {}", count);

        // Lire la première entrée d'image (16 octets) dans le répertoire d'images
        let mut entry = [0u8; 16];
        file.read_exact(&mut entry)?;

        // Taille des données de l'image
        let size = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]);
        // Offset où commence réellement l'image
        let offset = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]);
        println!("Image à l'offset {}, taille {}", offset, size);

        // Aller à l'offset pour lire les données brutes de l'image
        file.seek(SeekFrom::Start(offset as u64))?;
        let mut image_data = vec![0u8; size as usize];
        file.read_exact(&mut image_data)?;

        // Vérifier si l'image est au format PNG (en-tête PNG)
        if image_data.starts_with(&[0x89, b'P', b'N', b'G']) {
            println!("C'est une image PNG dans l'ICO.");
            // Le buffer image_data contient une image PNG complète
            // (À décoder avec une bibliothèque PNG si souhaité)
        } else {
            println!("C'est une image BMP brute (DIB) dans l'ICO.");

            // Extraire largeur depuis l’en-tête DIB
            let width =
                i32::from_le_bytes([image_data[4], image_data[5], image_data[6], image_data[7]])
                    as u32;
            // Hauteur de l'image multipliée par 2 (image + masque)
            let height_full =
                i32::from_le_bytes([image_data[8], image_data[9], image_data[10], image_data[11]]);
            // Hauteur réelle de l'image (la moitié supérieure)
            let height = (height_full / 2) as u32;
            // Nombre de bits par pixel (32 bits = 4 octets)
            let bpp = u16::from_le_bytes([image_data[14], image_data[15]]);

            if bpp != 32 {
                panic!("Seulement support 32 bits BMP non compressé ici");
            }

            // Taille de l’en-tête BMP (souvent 40)
            let header_size =
                u32::from_le_bytes([image_data[0], image_data[1], image_data[2], image_data[3]]);
            // Offset de départ des pixels réels
            let pixel_data_offset = header_size as usize;

            // Créer le buffer de destination pour l’image
            let mut buffer = PixelBuffer::new(width, height);

            // Parcourir les pixels ligne par ligne (ordre inversé car BMP est bottom-up)
            for y in 0..height {
                for x in 0..width {
                    // Calculer l'index dans image_data (source)
                    let px_index =
                        pixel_data_offset + (((height - 1 - y) * width + x) * 4) as usize;
                    let b = image_data[px_index]; // Canal bleu
                    let g = image_data[px_index + 1]; // Canal vert
                    let r = image_data[px_index + 2]; // Canal rouge
                    let a = image_data[px_index + 3]; // Alpha

                    // Calculer l'index dans le buffer de destination
                    let i = ((y * width + x) * 4) as usize;
                    // Copier le pixel dans l'ordre RGBA
                    buffer.pixels[i..i + 4].copy_from_slice(&[r, g, b, a]);
                }
            }

            // Retourner le PixelBuffer encodé avec succès
            return Ok(Some(buffer));
        }

        // Aucun buffer créé (ex: PNG non traité ici)
        return Ok(None);
    }
}
