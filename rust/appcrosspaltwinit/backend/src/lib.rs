#![allow(non_snake_case)]
#![allow(unused_parens)]

pub mod Image_manager;
// On déclare directement les modules en spécifiant le chemin vers leurs fichiers

#[path = "pixelManager\\mod.rs"]
pub mod PixelManager;
/*
#[path = "pixelBuffer\\elements.rs"]
mod element_mod;

pub use pixelbuffer_mod::PixelBuffer;*/
