#![allow(non_snake_case)]
#[cfg(target_arch = "wasm32")]
fn main() {
    // WebAssembly est appelé depuis JS, donc pas d'exécution directe.
    panic!("Ce binaire WebAssembly doit être appelé depuis JavaScript (voir frontend/web_demo)");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    #[cfg(feature = "web")]
    {
        use std::io;
        use std::process::Command;

        println!("Lancement du mode Web via navigateur…");

        let result = Command::new("basic-http-server")
            .args(&["-a", "127.0.0.1:8000"])
            .current_dir("../../web_demo") // <- adapte ce chemin si nécessaire
            .spawn();

        dbg!(&result);

        let mut child = result.expect("Impossible de démarrer basic-http-server");

        println!("Serveur démarré. Appuyez sur Entrée pour quitter...");
        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s);

        let _ = child.kill();
    }
    #[cfg(feature = "native")]
    {
        // Lancement du frontend natif sous Linux/Windows/macOS
        frontend::native::launch_new_app("app cross v0.1.0", 320, 200, true).unwrap();
    }
    /*
    #[cfg(not(any(feature = "web", feature = "native")))]
    compile_error!("Vous devez activer soit la feature 'web', soit la feature 'native'");*/
}
