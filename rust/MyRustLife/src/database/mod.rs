use akgine::database::{DataBase, DbError};
use std::env;
use std::path::PathBuf;

/// Where the SQLite file lives (platform-specific, handled by eframe).
pub fn dbPath() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // Android apps are sandboxed. Writable files must live under /data/data/<package_name>/files/.
        // We can extract the package name dynamically from Linux's /proc/self/cmdline pseudo-file.
        let package_name = std::fs::read_to_string("/proc/self/cmdline")
            .map(|s| {
                s.split('\0')
                    .next()
                    .unwrap_or("com.example.myrustlife")
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|_| "com.example.myrustlife".to_string());

        let mut path = PathBuf::from(format!("/data/data/{}/files", package_name));
        let _ = std::fs::create_dir_all(&path); // Ensure the app files directory is ready
        path.push("database.db");
        path
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, use the %LOCALAPPDATA% directory
        if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
            let mut path: PathBuf = PathBuf::from(local_app_data);
            path.push("MyRustLife");
            let _ = std::fs::create_dir_all(&path); // Create application folder if missing
            path.push("database.db");
            path
        } else {
            PathBuf::from("database.db") // Fallback to current working directory
        }
    }

    #[cfg(not(any(target_os = "android", target_os = "windows")))]
    {
        // On Linux/macOS, use the standard $HOME/.local/share directory
        if let Ok(home) = env::var("HOME") {
            let mut path = PathBuf::from(home);
            path.push(".local/share/MyRustLife");
            let _ = std::fs::create_dir_all(&path); // Create application folder if missing
            path.push("database.db");
            path
        } else {
            PathBuf::from("database.db") // Fallback to current working directory
        }
    }
}

/// Open the database. Call this ONCE at startup.
pub fn openDataBase() -> Result<DataBase, DbError> {
    DataBase::open(&dbPath())
    // This sets WAL mode, enables foreign keys, and creates the file if needed.
    // Tables are NOT created here — see Step 4.
}
