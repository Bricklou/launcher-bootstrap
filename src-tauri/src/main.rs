// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;

mod application;
mod deep_link;
mod constants;

fn main() {
    application::init().expect("Failed to initialize application");

    info!("Launcher bootstrap version {}", env!("CARGO_PKG_VERSION"));

    match application::run() {
        Err(e) => {
            eprintln!("Error while running tauri application: {}", e);
            std::process::exit(1);
        }
        _ => (),
    }
}
