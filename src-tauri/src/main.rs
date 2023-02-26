#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tracing::info;

mod application;
mod constants;
mod deep_link;
mod remote;

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
