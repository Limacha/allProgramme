#[path = "elements.rs"]
mod element_mod;

#[path = "PixelBuffer.rs"]
mod PixelBuffer_mod;

#[path = "fonction.rs"]
mod Fonction_mod;

pub use Fonction_mod::Fonction;
pub use PixelBuffer_mod::PixelBuffer;
pub use element_mod::{BufferElem, Button, DrawnFunc, InputField};
