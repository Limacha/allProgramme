pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
// #[cfg(debug_assertions)]
// pub const DEBUG_MODE: bool = true;
// #[cfg(not(debug_assertions))]
// pub const DEBUG_MODE: bool = false;
pub const PADDING: i8 = 5;
pub const TOP_PADDING: i8 = if cfg!(target_os = "android") { 25 } else { 0 };
pub const BOTTOM_PADDING: i8 = if cfg!(target_os = "android") { 40 } else { 0 };
