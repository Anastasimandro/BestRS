#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// The desktop entry point. The app itself lives in the shared library (`lib.rs`), so the
// Android crate can link the same modules without pulling egui or the desktop window code.
fn main() -> eframe::Result {
    ytm_rs::run()
}
