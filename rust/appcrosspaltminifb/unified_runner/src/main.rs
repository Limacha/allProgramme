#![allow(non_snake_case)]
#[cfg(target_arch = "wasm32")]
fn main() {
    // WebAssembly est appelé depuis JS, donc pas d'exécution directe.
    panic!("Ce binaire WebAssembly doit être appelé depuis JavaScript (voir frontend/web_demo)");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Lancement du frontend natif sous Linux/Windows/macOS
    frontend::native::start();
}
