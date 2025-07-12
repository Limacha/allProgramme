pub struct Fonction {}
impl Fonction {
    /// Inverse les composantes RGB d'une couleur
    ///
    /// # Arguments
    /// * `color` - Une couleur sous forme de tableau `[u8; 4]` (RGBA)
    ///
    /// # Retour
    /// * Une nouvelle couleur inversée `[u8; 4]`
    pub fn invert_color(color: [u8; 4]) -> [u8; 4] {
        [
            255 - color[0], // R
            255 - color[1], // G
            255 - color[2], // B
            color[3],       // A (inchangé)
        ]
    }
}
